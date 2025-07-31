mod context;
mod files;
mod package_tab;
mod package_tree;
mod question_tab;
mod round_tab;
mod storage;
mod theme_tab;
mod workarea;

use std::{collections::BTreeSet, path::PathBuf, sync::Arc};

use itertools::Itertools;
use log::error;
use opensi_core::prelude::*;

use crate::{
    app::{
        context::AppContext,
        files::FileLoader,
        storage::{EguiPackageBytesLoader, SharedPackageBytesStorage},
    },
    element::{ModalExt, ModalWrapper, empty_label},
    icon, icon_format, icon_str, icon_string, style,
};

pub const FONT_REGULAR_ID: &'static str = "regular";
pub const FONT_BOLD_ID: &'static str = "bold";

/// Main context for the whole app.
/// Serialized fields are saved and restored.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EditorApp {
    theme_name: String,
    show_tree: bool,
    show_properties: bool,
    recent_files: BTreeSet<PathBuf>,
    #[serde(skip)]
    package_state: PackageState,
    #[serde(skip)]
    storage: SharedPackageBytesStorage,
    #[serde(skip)]
    loaders: Vec<FileLoader>,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            package_state: PackageState::None,
            storage: SharedPackageBytesStorage::default(),
            theme_name: style::default_theme().name().to_string(),
            show_tree: true,
            show_properties: true,
            recent_files: BTreeSet::new(),
            loaders: vec![],
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
            theme.apply(&cc.egui_ctx);
        } else {
            error!("Unknown theme: {}", &app.theme_name);
            app.theme_name = style::default_theme().name().to_string();
            style::default_theme().apply(&cc.egui_ctx);
        }

        app
    }

    pub fn ctx(&mut self) -> AppContext {
        self.into()
    }
}

impl eframe::App for EditorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut loaders = std::mem::take(&mut self.loaders);
        loaders.retain_mut(|loader| !loader.update(self));
        self.loaders.extend(loaders);

        let mut new_pack_modal = ModalWrapper::new(ctx, "new-pack-modal");
        let mut authors_modal = ModalWrapper::new(ctx, "authors-modal");

        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(20, 8)))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Файл", |ui| {
                        if ui.button(icon_str!(FOLDER_SIMPLE_PLUS, "Новый")).clicked() {
                            match self.package_state {
                                PackageState::Active { .. } => {
                                    new_pack_modal.open();
                                },
                                _ => {
                                    self.package_state = PackageState::Active {
                                        package: Package::new(),
                                        selected: None,
                                    };
                                },
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(icon_str!(FOLDER_OPEN, "Открыть")).clicked() {
                            self.ctx().pick_new_package();
                            ui.close_menu();
                        }
                        if ui.button(icon_str!(FLOPPY_DISK_BACK, "Сохранить")).clicked() {
                            self.ctx().save_package();
                            ui.close_menu();
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            ui.menu_button(icon_str!(CLOCK_COUNTER_CLOCKWISE, "Недавние файлы"), |ui| {
                                if self.recent_files.is_empty() {
                                    empty_label(ui);
                                }
                                ui.set_min_width(200.0);
                                let to_open = self.recent_files.iter().find(|recent| {
                                    let Some(name) = recent.file_name().map(|filename| filename.to_string_lossy()) else {
                                        return false;
                                    };
                                    ui.button(egui::RichText::new(name).monospace()).clicked()
                                }).cloned();
                                if let Some(to_open) = to_open {
                                    self.ctx().load_new_package(to_open);
                                    ui.close_menu();
                                }
                            });

                            ui.separator();

                            ui.separator();
                            if ui.button("Выйти").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    });
                    if let PackageState::Active { .. } = self.package_state {
                        ui.menu_button("Пак", |ui| {
                            if ui.button(icon_str!(X, "Закрыть")).clicked() {
                                self.package_state = PackageState::None;
                                ui.close_menu();
                            }
                        });
                    }

                    ui.menu_button("Настройки", |ui| {
                        ui.menu_button(icon_str!(SWATCHES, "Тема"), |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Текущая тема:");
                                ui.label(egui::RichText::new(&self.theme_name));
                            });

                            ui.separator();

                            for theme in style::all_themes() {
                                let title = if theme.color_scheme().dark {
                                    icon_string!(MOON, theme.name())
                                } else {
                                    icon_string!(SUN, theme.name())
                                };

                                if ui.button(title).clicked() {
                                    self.theme_name = theme.name().to_string();
                                    theme.apply(ui.ctx());
                                    if let Some(storage) = frame.storage_mut() {
                                        self.save(storage);
                                    }
                                }
                            }
                        });
                    });

                    ui.menu_button("Справка", |ui| {
                        if ui.button(icon_str!(STUDENT, "Авторы")).clicked() {
                            authors_modal.open();
                            ui.close_menu();
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.toggle_value(&mut self.show_properties, icon!(LIST_BULLETS)).on_hover_text("Включить/выключить правую панель с параметрами выбранного элемента");
                        ui.toggle_value(&mut self.show_tree, icon!(TREE_VIEW)).on_hover_text(
                            "Включить/выключить левую панель с деревом пакета вопросов",
                        );
                    });
                });
            });

        if let PackageState::Active { package, selected } = &mut self.package_state {
            egui::SidePanel::right("properties-list-side")
                .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(20))
                .width_range(280.0..=400.0)
                .show_animated(ctx, self.show_properties, |ui| {
                    workarea::properties(package, selected, ui);
                });

            egui::SidePanel::left("question-tree-side")
                .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(20))
                .width_range(280.0..=400.0)
                .max_width(400.0)
                .show_animated(ctx, self.show_tree, |ui| {
                    package_tree::package_tree(package, selected, ui);
                });
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(egui::Margin::symmetric(40, 20))
                    .fill(ctx.style().visuals.widgets.noninteractive.weak_bg_fill),
            )
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        if let PackageState::Active { package, selected } = &mut self.package_state
                        {
                            workarea::workarea(package, selected, ui);
                        } else {
                            let text =
                                egui::RichText::new(icon_str!(GRADUATION_CAP, "OpenSI Editor"))
                                    .size(64.0)
                                    .color(ui.style().visuals.weak_text_color());
                            ui.add(egui::Label::new(text).selectable(false));
                        }
                    },
                );
            });

        new_pack_modal.show(ctx, |ui| {
            ui.modal_title(icon_str!(PENCIL_SIMPLE_LINE, "Перезаписать текущий пак ?"));
            ui.modal_buttons(|ui| {
                if ui.modal_danger(icon_str!(PROHIBIT, "Отмена")).clicked() {}
                if ui.modal_confirm(icon_str!(CHECK, "Перезаписать")).clicked() {
                    self.package_state =
                        PackageState::Active { package: Package::new(), selected: None };
                }
            });
        });

        authors_modal.show(ctx, |ui| {
            ui.modal_title(icon_str!(GRADUATION_CAP, "OpenSI Editor"));
            ui.horizontal(|ui| {
                ui.strong("Авторы:");
                let authors = env!("CARGO_PKG_AUTHORS").split(":").join(", ");
                ui.label(authors);
            });

            ui.horizontal(|ui| {
                ui.strong("Версия:");
                let version = env!("CARGO_PKG_VERSION");
                ui.code(format!("v{version}"));
            });

            ui.horizontal(|ui| {
                ui.strong("Репозиторий:");
                let url = env!("CARGO_PKG_REPOSITORY");
                let short_url = url.trim_start_matches("https://github.com/");
                ui.hyperlink_to(icon_format!(GITHUB_LOGO, "{short_url}"), url);
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.weak("Сделано с помощью");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            });

            ui.modal_buttons(|ui| {
                ui.modal_button(icon_str!(X, "Закрыть"));
            });
        });
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
