use opensi_core::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum CardKind<'a> {
    Theme(&'a Theme),
    Question(&'a Question),
    New,
}

impl<'a> CardKind<'a> {
    pub fn show(&self, ui: &mut egui::Ui) -> egui::Response {
        let (text, fill, text_color) = match self {
            CardKind::Theme(theme) => (
                theme.name.clone(),
                ui.visuals().widgets.active.bg_fill,
                ui.visuals().widgets.active.text_color(),
            ),
            CardKind::Question(question) => (
                question.price.to_string(),
                egui::Color32::TRANSPARENT,
                ui.visuals().widgets.inactive.text_color(),
            ),
            CardKind::New => (
                "➕ Новый вопрос".to_string(),
                egui::Color32::TRANSPARENT,
                ui.visuals().weak_text_color(),
            ),
        };
        let mut frame = egui::Frame::default()
            .inner_margin(16.0)
            .outer_margin(egui::Margin::symmetric(0.0, 4.0))
            .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
            .rounding(8.0)
            .fill(fill)
            .begin(ui);

        // aprox
        let font_size = 22.0 - (text.len() as isize - 8).max(0) as f32 * 0.3;
        frame.content_ui.add_sized(
            egui::vec2(100.0, 0.0),
            egui::Label::new(egui::RichText::new(&text).size(font_size).color(text_color))
                .selectable(false)
                .halign(egui::Align::Center)
                .wrap(),
        );
        let rect =
            frame.content_ui.min_rect() + frame.frame.inner_margin + frame.frame.outer_margin;
        let response = ui.allocate_rect(rect, egui::Sense::click());
        if response.hovered() {
            frame.frame.stroke = ui.style().visuals.widgets.active.bg_stroke;
            frame.frame.fill = ui.style().visuals.widgets.active.weak_bg_fill;
        }
        frame.paint(ui);
        response.on_hover_text(&text)
    }
}
