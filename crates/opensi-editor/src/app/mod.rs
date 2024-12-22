mod file_dialogs;
mod package_tab;
mod package_tree;
mod question_tab;
mod round_tab;
mod theme_tab;
mod workarea;

use itertools::Itertools;
use opensi_core::prelude::*;

use crate::app::file_dialogs::LoadingPackageReceiver;

const FONT_REGULAR_ID: &'static str = "Regular";
const FONT_ITALIC_ID: &'static str = "Italic";

/// Main context for the whole app.
/// Serialized fields are saved and restored.
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
#[serde(default)]
pub struct EditorApp {
    package_state: PackageState,
}

impl EditorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let mut font_definitions = egui::FontDefinitions::default();
        font_definitions.font_data.insert(
            FONT_REGULAR_ID.into(),
            egui::FontData::from_static(include_bytes!("../../assets/ui/Lora-Medium.ttf")),
        );
        font_definitions.font_data.insert(
            FONT_ITALIC_ID.into(),
            egui::FontData::from_static(include_bytes!("../../assets/ui/Lora-MediumItalic.ttf")),
        );
        if let Some(family) = font_definitions.families.get_mut(&egui::FontFamily::Proportional) {
            family.insert(0, FONT_REGULAR_ID.into());
        }
        cc.egui_ctx.set_fonts(font_definitions);

        app
    }
}

impl eframe::App for EditorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.package_state.update();

        let new_pack_modal =
            egui_modal::Modal::new(ctx, "new-pack-modal").with_close_on_outside_click(true);
        new_pack_modal.show(|ui| {
            new_pack_modal.title(ui, "Перезаписать текущий пак ?");
            new_pack_modal.frame(ui, |ui| {
                new_pack_modal
                    .body(ui, "Создание нового пака перезапишет текущий пак. Вы уверены ?");
            });
            new_pack_modal.buttons(ui, |ui| {
                if new_pack_modal.caution_button(ui, "Отмена").clicked() {
                    new_pack_modal.close();
                };
                if new_pack_modal.suggested_button(ui, "Перезаписать").clicked() {
                    self.package_state =
                        PackageState::Active { package: Package::new(), selected: None };
                };
            });
        });

        let authors_modal =
            egui_modal::Modal::new(ctx, "authors-modal").with_close_on_outside_click(true);
        authors_modal.show(|ui| {
            authors_modal.title(ui, "🎓 OpenSI Editor");
            authors_modal.frame(ui, |ui| {
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
                    ui.hyperlink_to(format!(" {short_url}"), url);
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.weak("Сделано с помощью");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                });
            });
            authors_modal.buttons(ui, |ui| {
                authors_modal.button(ui, "Закрыть");
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                ui.add_space(16.0);
                ui.menu_button("Файл", |ui| {
                    if ui.button("➕ Новый пак").clicked() {
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
                    if ui.button("⮩ Открыть").clicked() {
                        let package_receiver = file_dialogs::import_dialog();
                        self.package_state = PackageState::Loading(package_receiver);
                        ui.close_menu();
                    }
                    if ui.button("💾 Сохранить").clicked() {
                        let PackageState::Active { ref package, .. } = self.package_state else {
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
                        if ui.button("❌ Закрыть").clicked() {
                            self.package_state = PackageState::None;
                            ui.close_menu();
                        }
                    });
                }

                ui.menu_button("Справка", |ui| {
                    if ui.button("💬 Авторы").clicked() {
                        authors_modal.open();
                        ui.close_menu();
                    }
                });
            });
        });

        if let PackageState::Active { package, selected } = &mut self.package_state {
            egui::SidePanel::left("question-tree").min_width(300.0).show(ctx, |ui| {
                package_tree::package_tree(package, selected, ui);
            });
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        if let PackageState::Active { package, selected } = &mut self.package_state
                        {
                            workarea::workarea(package, selected, ui);
                        } else {
                            let text = egui::RichText::new("OpenSI Editor")
                                .italics()
                                .size(64.0)
                                .color(ui.style().visuals.weak_text_color());
                            ui.add(egui::Label::new(text).selectable(false));
                        }
                    },
                );
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
            Self::Loading(receiver) => {
                match receiver.try_recv() {
                    Ok(Ok(package)) => {
                        *self = Self::Active { package, selected: None };
                    },
                    Ok(Err(_err)) => {
                        // TODO: error handle
                    },
                    Err(_) => {},
                }
            },
            _ => {},
        }
    }
}
