use crate::{
    file_dialogs::{self, LoadingPackageReceiver},
    package_tree::{self},
};

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
            egui::FontData::from_static(include_bytes!("../assets/ui/Lora-Medium.ttf")),
        );
        font_definitions.font_data.insert(
            FONT_ITALIC_ID.into(),
            egui::FontData::from_static(include_bytes!("../assets/ui/Lora-MediumItalic.ttf")),
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.add_space(16.0);
                ui.menu_button("Файл", |ui| {
                    if ui.button("⮩ Импорт").clicked() {
                        let package_receiver = file_dialogs::import_dialog();
                        self.package_state = PackageState::Loading(package_receiver);
                        ui.close_menu();
                    }
                    if ui.button("💾 Сохранить").clicked() {
                        let PackageState::Active(ref package) = self.package_state else {
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
                if let PackageState::Active(ref _package) = self.package_state {
                    ui.menu_button("Пак", |ui| {
                        if ui.button("❌Закрыть").clicked() {
                            self.package_state = PackageState::None;
                            ui.close_menu();
                        }
                    });
                }
            });
        });

        egui::SidePanel::left("question-tree").min_width(200.0).show(ctx, |ui| {
            match self.package_state {
                PackageState::Active(ref mut package) => {
                    package_tree::package_tree(package, ui);
                },
                _ => {
                    ui.weak("Пак не выбран");
                },
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    let text = egui::RichText::new("We're so back")
                        .size(100.0)
                        .color(ui.style().visuals.weak_text_color());
                    ui.add(egui::Label::new(text));
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
    Active(opensi_core::Package),
}

impl PackageState {
    fn update(&mut self) {
        match self {
            Self::Loading(receiver) => {
                match receiver.try_recv() {
                    Ok(Ok(package)) => {
                        *self = Self::Active(package);
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
