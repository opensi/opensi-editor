use dirs::home_dir;
use rfd::AsyncFileDialog;

#[cfg(not(target_arch = "wasm32"))]
use tokio;
#[cfg(target_arch = "wasm32")]
use tokio_with_wasm::alias as tokio;

const FONT_REGULAR_ID: &'static str = "Regular";
const FONT_ITALIC_ID: &'static str = "Italic";

/// Main context for the whole app.
/// Serialized fields are saved and restored.
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
#[serde(default)]
pub struct EditorApp {}

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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.add_space(16.0);
                ui.menu_button("Файл", |ui| {
                    if ui.button("⮩ Импорт").clicked() {
                        let _result = tokio::spawn(async {
                            let home_dir = home_dir().unwrap();
                            let file = AsyncFileDialog::new()
                                .set_title("Выбрать файл с вопросами для импорта")
                                .add_filter("SIGame Pack", &["siq"])
                                .set_directory(home_dir)
                                .set_can_create_directories(false)
                                .pick_file()
                                .await?;

                            Some(file.read().await)
                        });
                        // TODO
                    }
                    if ui.button("💾 Сохранить").clicked() {
                        let _result = tokio::spawn(async {
                            let file = AsyncFileDialog::new()
                                .set_title("Сохранить выбранный пакет с вопросами")
                                .set_directory("/")
                                .set_file_name("pack.siq")
                                .save_file()
                                .await?;

                            let data = [0];
                            file.write(&data).await.ok()
                        });
                        // TODO
                    }

                    if !cfg!(target_arch = "wasm32") {
                        ui.separator();
                        if ui.button("Выйти").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
            });
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
