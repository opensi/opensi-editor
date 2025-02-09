mod file_dialogs;
mod package_tab;
mod package_tree;
mod question_tab;
mod round_tab;
mod theme_tab;
mod workarea;

use itertools::Itertools;
use log::error;
use opensi_core::prelude::*;

use crate::{
    app::file_dialogs::LoadingPackageReceiver,
    element::{ModalExt, ModalWrapper},
    icon_format, icon_str, icon_string, style,
};

const FONT_REGULAR_ID: &'static str = "regular";

/// Main context for the whole app.
/// Serialized fields are saved and restored.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct EditorApp {
    package_state: PackageState,
    theme_name: String,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            package_state: PackageState::None,
            theme_name: style::default_theme().name().to_string(),
        }
    }
}

impl EditorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value::<Vec<u8>>(storage, eframe::APP_KEY)
                .and_then(|binary| bincode::deserialize(&binary).ok())
                .unwrap_or_default()
        } else {
            Default::default()
        };

        if let Some(theme) = style::choose(&app.theme_name) {
            theme.apply(&cc.egui_ctx);
        } else {
            error!("Unknown theme: {}", &app.theme_name);
            app.theme_name = style::default_theme().name().to_string();
            style::default_theme().apply(&cc.egui_ctx);
        }

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            FONT_REGULAR_ID.into(),
            egui::FontData::from_static(include_bytes!("../../assets/fonts/Rubik-Medium.ttf"))
                .into(),
        );
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            family.insert(0, FONT_REGULAR_ID.into());
        }

        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Fill);

        cc.egui_ctx.set_fonts(fonts);

        app
    }
}

impl eframe::App for EditorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        match bincode::serialize(self) {
            Ok(binary) => {
                eframe::set_value(storage, eframe::APP_KEY, &binary);
            },
            Err(err) => error!("Unable to bincode app state: {err}"),
        }
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.package_state.update();

        let mut new_pack_modal = ModalWrapper::new(ctx, "new-pack-modal");
        let mut authors_modal = ModalWrapper::new(ctx, "authors-modal");

        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(4.0))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.add_space(16.0);
                    ui.menu_button("Файл", |ui| {
                        if ui.button(icon_str!(FOLDER_SIMPLE_PLUS, "Новый пак")).clicked() {
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
                            let package_receiver = file_dialogs::import_dialog();
                            self.package_state = PackageState::Loading(package_receiver);
                            ui.close_menu();
                        }
                        if ui.button(icon_str!(FLOPPY_DISK_BACK, "Сохранить")).clicked() {
                            let PackageState::Active { ref package, .. } = self.package_state
                            else {
                                return;
                            };
                            file_dialogs::export_dialog(package);
                            ui.close_menu();
                        }

                        if !cfg!(target_arch = "wasm32") {
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

                    ui.menu_button("Справка", |ui| {
                        if ui.button(icon_str!(STUDENT, "Авторы")).clicked() {
                            authors_modal.open();
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Настройки", |ui| {
                        ui.menu_button(icon_str!(SWATCHES, "Тема"), |ui| {
                            ui.label(format!("Текущая тема: {}", self.theme_name));

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
                });
            });

        if let PackageState::Active { package, selected } = &mut self.package_state {
            egui::SidePanel::left("question-tree")
                .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(20.0))
                .min_width(320.0)
                .show(ctx, |ui| {
                    package_tree::package_tree(package, selected, ui);
                });
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(20.0)
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

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
enum PackageState {
    #[default]
    None,
    #[serde(skip)]
    Loading(LoadingPackageReceiver),
    Active {
        package: Package,
        selected: Option<PackageNode>,
    },
}

impl PackageState {
    fn update(&mut self) {
        match self {
            Self::Loading(receiver) => match receiver.try_recv() {
                Ok(Ok(package)) => {
                    *self = Self::Active { package, selected: None };
                },
                Ok(Err(err)) => {
                    error!("Error loading package: {err}");
                },
                Err(_) => {},
            },
            _ => {},
        }
    }
}
