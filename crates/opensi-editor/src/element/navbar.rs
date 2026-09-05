use crate::{
    element::common::icon_button,
    icon,
    style::{METRICS, mix},
};

pub fn style_navbar_menus(ui: &mut egui::Ui) {
    let strong = ui.visuals().strong_text_color();
    let weak = ui.visuals().weak_text_color();
    let hover_bg = mix(ui.visuals().panel_fill, ui.visuals().text_color(), 0.10);

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.spacing_mut().button_padding = egui::vec2(METRICS.padding, METRICS.padding_small);

    let w = &mut ui.visuals_mut().widgets;
    w.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    w.inactive.bg_fill = egui::Color32::TRANSPARENT;
    w.inactive.bg_stroke = egui::Stroke::NONE;
    w.inactive.corner_radius = egui::CornerRadius::same(0);
    w.inactive.fg_stroke.color = weak;
    for wv in [&mut w.hovered, &mut w.active, &mut w.open] {
        wv.weak_bg_fill = hover_bg;
        wv.bg_fill = hover_bg;
        wv.bg_stroke = egui::Stroke::NONE;
        wv.corner_radius = egui::CornerRadius::same(0);
        wv.fg_stroke.color = strong;
    }
}

pub fn navbar_logo(ui: &mut egui::Ui, full_height: egui::Rangef) -> egui::Response {
    let accent = ui.visuals().selection.stroke.color;
    let strong = ui.visuals().strong_text_color();
    let border = ui.visuals().widgets.noninteractive.bg_stroke.color;

    ui.add_space(METRICS.padding);
    let cap = ui.add(
        egui::Label::new(
            egui::RichText::new(icon!(GRADUATION_CAP)).size(METRICS.font_large).color(accent),
        )
        .selectable(false)
        .sense(egui::Sense::click()),
    );
    ui.add_space(METRICS.gap);
    let text = ui.add(
        egui::Label::new(
            egui::RichText::new("OpenSI").strong().size(METRICS.font_body).color(strong),
        )
        .selectable(false)
        .sense(egui::Sense::click()),
    );
    ui.add_space(METRICS.padding);
    let sep_x = ui.cursor().min.x;
    ui.painter().vline(sep_x, full_height, egui::Stroke::new(1.0, border));

    (cap | text).on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A neutral, bordered compact header button (right side of the navbar).
pub fn header_icon_button(ui: &mut egui::Ui, glyph: &str, active: bool) -> egui::Response {
    let visuals = ui.visuals();
    let accent = visuals.selection.stroke.color;
    let strong = visuals.strong_text_color();
    let weak = visuals.weak_text_color();
    let fill = visuals.widgets.inactive.bg_fill;
    let (color, hover_color) = if active { (accent, accent) } else { (weak, strong) };
    icon_button(
        ui,
        egui::vec2(METRICS.compact_size + METRICS.gap_tiny, METRICS.compact_size),
        glyph,
        color,
        hover_color,
        fill,
    )
}
