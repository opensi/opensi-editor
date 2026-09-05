mod context;
mod files;
mod modal;
mod package_tab;
mod package_tree;
mod question_tab;
mod round_tab;
mod storage;
mod theme_tab;
mod welcome;
mod workarea;

use std::{collections::BTreeSet, path::PathBuf, sync::Arc};

use log::error;
use opensi_core::prelude::*;

use crate::{
    app::{
        context::{AppContext, PackageContext},
        files::FilesQueue,
        modal::about_modal,
        storage::{EguiPackageBytesLoader, SharedPackageBytesStorage},
    },
    element::{
        ModalWrapper, empty_label,
        menu::{menu_divider, menu_item, menu_label, style_menu},
        navbar::{header_icon_button, navbar_logo, style_navbar_menus},
    },
    icon, style,
};

pub const FONT_REGULAR_ID: &'static str = "regular";
pub const FONT_BOLD_ID: &'static str = "bold";

/// Main context for the whole app.
/// Serialized fields are saved and restored.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EditorApp {
    theme_name: String,
    color_mode: style::ColorMode,
    show_tree: bool,
    show_properties: bool,
    recent_files: BTreeSet<PathBuf>,
    #[serde(skip)]
    package_state: PackageState,
    #[serde(skip)]
    storage: SharedPackageBytesStorage,
    #[serde(skip)]
    files_queue: Vec<FilesQueue>,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            package_state: PackageState::None,
            storage: SharedPackageBytesStorage::default(),
            theme_name: style::default_theme().name().to_string(),
            color_mode: style::ColorMode::default(),
            show_tree: true,
            show_properties: true,
            recent_files: BTreeSet::new(),
            files_queue: vec![],
        }
    }
}

impl EditorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            FONT_REGULAR_ID.into(),
            egui::FontData::from_static(include_bytes!("../../assets/fonts/Manrope-Regular.ttf"))
                .into(),
        );
        fonts.font_data.insert(
            FONT_BOLD_ID.into(),
            egui::FontData::from_static(include_bytes!("../../assets/fonts/Manrope-SemiBold.ttf"))
                .into(),
        );
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            family.insert(0, FONT_REGULAR_ID.into());
        }
        fonts
            .families
            .insert(egui::FontFamily::Name(FONT_BOLD_ID.into()), vec![FONT_BOLD_ID.into()]);

        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.add_bytes_loader(Arc::new(EguiPackageBytesLoader::new(&app.storage)));

        if let Some(theme) = style::choose(&app.theme_name) {
            theme.apply(&cc.egui_ctx, app.color_mode);
        } else {
            error!("Unknown theme: {}", &app.theme_name);
            app.theme_name = style::default_theme().name().to_string();
            style::default_theme().apply(&cc.egui_ctx, app.color_mode);
        }

        app
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let theme = style::choose(&self.theme_name).unwrap_or_else(style::default_theme);
        theme.apply(ctx, self.color_mode);
    }

    pub fn ctx(&mut self) -> AppContext {
        self.into()
    }

    pub fn package_ctx(&mut self) -> Option<PackageContext> {
        PackageContext::try_new(self)
    }

    pub fn has_active_package(&self) -> bool {
        matches!(self.package_state, PackageState::Active { .. })
    }
}

impl eframe::App for EditorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut files_queue = std::mem::take(&mut self.files_queue);
        files_queue.retain_mut(|queue| !queue.update(self));
        self.files_queue.extend(files_queue);

        let mut new_pack_modal = ModalWrapper::new(ctx, "new-pack-modal");
        let mut authors_modal = ModalWrapper::new(ctx, "authors-modal");

        egui::TopBottomPanel::top("top_panel")
            .exact_height(style::METRICS.navbar_height)
            .frame(egui::Frame::new().fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| {
                let bar = ui.max_rect();
                let border = ui.visuals().widgets.noninteractive.bg_stroke.color;
                ui.painter().hline(
                    bar.x_range(),
                    bar.bottom() - 0.5,
                    egui::Stroke::new(1.0, border),
                );

                ui.horizontal_centered(|ui| {
                    style_navbar_menus(ui);

                    if navbar_logo(ui, bar.y_range()).clicked() {
                        authors_modal.open();
                    }

                    ui.menu_button("Файл", |ui| {
                        style_menu(ui);
                        if menu_item(ui, icon!(FOLDER_SIMPLE_PLUS), "Новый пакет", "").clicked()
                        {
                            match self.package_state {
                                PackageState::Active { .. } => {
                                    new_pack_modal.open();
                                },
                                _ => self.ctx().new_package(),
                            }
                            ui.close_menu();
                        }
                        if menu_item(ui, icon!(FOLDER_OPEN), "Открыть пакет", "").clicked()
                        {
                            self.ctx().pick_new_package();
                            ui.close_menu();
                        }
                        if menu_item(ui, icon!(FLOPPY_DISK_BACK), "Сохранить", "").clicked()
                        {
                            self.ctx().save_package();
                            ui.close_menu();
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            menu_divider(ui);
                            menu_label(ui, icon!(CLOCK_COUNTER_CLOCKWISE), "Недавние файлы");
                            self.recent_files.retain(|recent| recent.exists());
                            if self.recent_files.is_empty() {
                                empty_label(ui);
                            } else {
                                let to_open = self.recent_files.iter().find_map(|recent| {
                                    let name = recent.file_name()?.to_string_lossy();
                                    menu_item(ui, icon!(FILE), name.as_ref(), "")
                                        .clicked()
                                        .then(|| recent.clone())
                                });
                                if let Some(to_open) = to_open {
                                    self.ctx().load_new_package(to_open);
                                    ui.close_menu();
                                }
                            }

                            menu_divider(ui);
                            if menu_item(ui, icon!(SIGN_OUT), "Выйти", "").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    });

                    if let PackageState::Active { .. } = self.package_state {
                        ui.menu_button("Пак", |ui| {
                            style_menu(ui);
                            if menu_item(ui, icon!(X), "Закрыть пакет", "").clicked() {
                                self.package_state = PackageState::None;
                                ui.close_menu();
                            }
                        });
                    }

                    ui.menu_button("Настройки", |ui| {
                        style_menu(ui);

                        for mode in [style::ColorMode::Day, style::ColorMode::Night] {
                            let hint = if self.color_mode == mode { icon!(CHECK) } else { "" };
                            if menu_item(ui, mode.glyph(), &mode.to_string(), hint).clicked() {
                                self.color_mode = mode;
                                self.apply_theme(ui.ctx());
                                if let Some(storage) = frame.storage_mut() {
                                    self.save(storage);
                                }
                            }
                        }

                        menu_divider(ui);

                        for theme in style::all_themes() {
                            let hint =
                                if theme.name() == self.theme_name { icon!(CHECK) } else { "" };
                            if menu_item(ui, icon!(SWATCHES), theme.name(), hint).clicked() {
                                self.theme_name = theme.name().to_string();
                                theme.apply(ui.ctx(), self.color_mode);
                                if let Some(storage) = frame.storage_mut() {
                                    self.save(storage);
                                }
                            }
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.reset_style();
                        ui.add_space(style::METRICS.gap);
                        if header_icon_button(ui, icon!(LIST_BULLETS), self.show_properties)
                            .on_hover_text("Правая панель со свойствами")
                            .clicked()
                        {
                            self.show_properties = !self.show_properties;
                        }
                        ui.add_space(style::METRICS.gap_tiny);
                        if header_icon_button(ui, icon!(TREE_VIEW), self.show_tree)
                            .on_hover_text("Левая панель с деревом пакета")
                            .clicked()
                        {
                            self.show_tree = !self.show_tree;
                        }
                    });
                });
            });

        if self.has_active_package() {
            let sidebar_fill = ctx.style().visuals.faint_bg_color;
            egui::SidePanel::right("properties-list-side")
                .frame(egui::Frame::new().fill(sidebar_fill))
                .default_width(340.0)
                .width_range(300.0..=440.0)
                .show_animated(ctx, self.show_properties, |ui| {
                    if let Some(mut pkg_ctx) = self.package_ctx() {
                        workarea::properties(&mut pkg_ctx, ui);
                    }
                });

            egui::SidePanel::left("question-tree-side")
                .frame(egui::Frame::new().fill(sidebar_fill))
                .default_width(268.0)
                .width_range(240.0..=400.0)
                .show_animated(ctx, self.show_tree, |ui| {
                    if let Some(mut pkg_ctx) = self.package_ctx() {
                        package_tree::package_tree(&mut pkg_ctx, ui);
                    }
                });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| {
                if let Some(mut ctx) = self.package_ctx() {
                    workarea::workarea(&mut ctx, ui);
                } else {
                    welcome::welcome_page(&mut self.ctx(), ui);
                }
            });

        new_pack_modal.show(ctx, |ui| {
            if modal::new_pack_modal(ui) {
                self.ctx().new_package();
            }
        });

        authors_modal.show(ctx, about_modal);
    }
}

#[derive(Default, Debug)]
enum PackageState {
    #[default]
    None,
    Active {
        package: Package,
        selected: Option<PackageNode>,
    },
}
