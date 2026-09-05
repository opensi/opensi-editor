use crate::style;

struct MenuPalette {
    glyph: egui::Color32,
    label: egui::Color32,
    hint: egui::Color32,
    hover: egui::Color32,
    divider: egui::Color32,
}

impl MenuPalette {
    fn of(ui: &egui::Ui) -> Self {
        let scheme = style::current_scheme(ui.ctx());
        MenuPalette {
            glyph: scheme.text_weak,
            label: scheme.text,
            hint: scheme.text_weak,
            hover: scheme.hover_fill(),
            divider: scheme.border(),
        }
    }
}

pub fn style_menu(ui: &mut egui::Ui) {
    ui.set_min_width(style::METRICS.menu_width);
    ui.set_max_width(style::METRICS.menu_width);
    ui.spacing_mut().item_spacing.y = style::METRICS.hairline;
}

/// Paint one [glyph | label | hint] row with a hover fill.
fn menu_row(
    ui: &mut egui::Ui,
    glyph: &str,
    label: &str,
    hint: &str,
    glyph_color: egui::Color32,
    label_color: egui::Color32,
    hint_color: egui::Color32,
    hover: egui::Color32,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(style::METRICS.menu_width, style::METRICS.menu_row_height),
        egui::Sense::click(),
    );
    if response.hovered() {
        ui.painter().rect_filled(rect, egui::CornerRadius::same(style::METRICS.rounding), hover);
    }
    let cy = rect.center().y;
    let pad = style::METRICS.gap;
    ui.painter().text(
        egui::pos2(rect.left() + pad + style::METRICS.gap, cy),
        egui::Align2::CENTER_CENTER,
        glyph,
        egui::FontId::proportional(style::METRICS.font_icon),
        glyph_color,
    );
    ui.painter().text(
        egui::pos2(rect.left() + pad + style::METRICS.compact_size, cy),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(style::METRICS.font_body),
        label_color,
    );
    if !hint.is_empty() {
        ui.painter().text(
            egui::pos2(rect.right() - pad, cy),
            egui::Align2::RIGHT_CENTER,
            hint,
            egui::FontId::proportional(style::METRICS.font_small),
            hint_color,
        );
    }
    response
}

/// A dropdown-menu row: [glyph | label | hint], full-width with a hover fill.
pub fn menu_item(ui: &mut egui::Ui, glyph: &str, label: &str, hint: &str) -> egui::Response {
    let p = MenuPalette::of(ui);
    menu_row(ui, glyph, label, hint, p.glyph, p.label, p.hint, p.hover)
}

/// A destructive dropdown-menu row (glyph and label painted in the error color).
pub fn menu_item_danger(ui: &mut egui::Ui, glyph: &str, label: &str, hint: &str) -> egui::Response {
    let p = MenuPalette::of(ui);
    let danger = ui.visuals().error_fg_color;
    menu_row(ui, glyph, label, hint, danger, danger, p.hint, p.hover)
}

/// A thin, inset divider between menu groups.
pub fn menu_divider(ui: &mut egui::Ui) {
    let divider = MenuPalette::of(ui).divider;
    ui.add_space(style::METRICS.gap_tiny);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(style::METRICS.menu_width, style::METRICS.hairline),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        (rect.left() + style::METRICS.gap_small)..=(rect.right() - style::METRICS.gap_small),
        rect.center().y,
        egui::Stroke::new(1.0, divider),
    );
    ui.add_space(style::METRICS.gap_tiny);
}

/// A non-interactive menu section header: a muted [glyph | label] row.
pub fn menu_label(ui: &mut egui::Ui, glyph: &str, label: &str) {
    let palette = MenuPalette::of(ui);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(style::METRICS.menu_width, style::METRICS.compact_size),
        egui::Sense::hover(),
    );
    let cy = rect.center().y;
    let pad = style::METRICS.gap;
    ui.painter().text(
        egui::pos2(rect.left() + pad + style::METRICS.gap, cy),
        egui::Align2::CENTER_CENTER,
        glyph,
        egui::FontId::proportional(style::METRICS.font_body),
        palette.hint,
    );
    ui.painter().text(
        egui::pos2(rect.left() + pad + style::METRICS.compact_size, cy),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(style::METRICS.font_label),
        palette.hint,
    );
}
