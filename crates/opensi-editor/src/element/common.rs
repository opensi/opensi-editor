use crate::style::mix;

#[macro_export]
macro_rules! icon {
    ($icon:ident) => {
        egui_phosphor::regular::$icon
    };
}

#[macro_export]
macro_rules! icon_str {
    ($icon:ident, $str:literal) => {
        const_format::formatcp!("{} {}", crate::icon!($icon), $str)
    };
}

#[macro_export]
macro_rules! icon_string {
    ($icon:ident, $string:expr) => {
        format!("{} {}", crate::icon!($icon), $string)
    };
}

#[macro_export]
macro_rules! icon_format {
    ($icon:ident, $fmt:literal, $($args:tt)*) => {
        format!("{} {}", crate::icon!($icon), format_args!($fmt, $($args)*))
    };
    ($icon:ident, $fmt:literal) => {
        format!("{} {}", crate::icon!($icon), format_args!($fmt))
    };
}

/// Paint a dashed rectangular border just inside `rect`, dashed outline
/// every dashed element (add buttons, chip add-fields) is drawn with.
pub fn dashed_border(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let r = rect.shrink(0.5);
    let points = [r.left_top(), r.right_top(), r.right_bottom(), r.left_bottom(), r.left_top()];
    painter.extend(egui::Shape::dashed_line(&points, egui::Stroke::new(1.0_f32, color), 4.0, 4.0));
}

/// A dashed "+ ..." button over an exact `size`.
pub fn dashed_button(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    label: &str,
    font_size: f32,
    left_pad: Option<f32>,
) -> egui::Response {
    let strong = ui.visuals().strong_text_color();
    let weak = ui.visuals().weak_text_color();
    let border = ui.visuals().widgets.noninteractive.bg_stroke.color;
    let border_strong = ui.visuals().widgets.hovered.bg_stroke.color;
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let hovered = response.hovered();
    let (fg, edge) = if hovered { (strong, border_strong) } else { (weak, border) };
    dashed_border(ui.painter(), rect, edge);
    let font = egui::FontId::proportional(font_size);
    match left_pad {
        Some(pad) => {
            ui.painter().text(
                egui::pos2(rect.left() + pad, rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                font,
                fg,
            );
        },
        None => {
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, label, font, fg);
        },
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A dashed "+ ..." chip sized to its label (inline add affordances).
pub fn dashed_chip(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let font = egui::FontId::proportional(crate::style::METRICS.font_label);
    let color = ui.visuals().weak_text_color();
    let galley_size = ui.fonts(|f| f.layout_no_wrap(label.to_owned(), font, color)).size();
    dashed_button(
        ui,
        galley_size + egui::vec2(crate::style::METRICS.card_padding, crate::style::METRICS.padding),
        label,
        crate::style::METRICS.font_label,
        None,
    )
}

/// A full-width dashed "+ ..." add row .
pub fn dashed_add_row(ui: &mut egui::Ui, label: &str, centered: bool) -> egui::Response {
    let metrics = &crate::style::METRICS;
    let size = egui::vec2(ui.available_width(), metrics.add_row_height);
    let left_pad = (!centered).then_some(metrics.padding);
    dashed_button(ui, size, label, metrics.font_body, left_pad)
}

/// A small bordered icon button.
pub fn icon_button(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    glyph: &str,
    color: egui::Color32,
    hover_color: egui::Color32,
    fill: egui::Color32,
) -> egui::Response {
    let m = &crate::style::METRICS;
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let hovered = response.hovered();
    let border =
        if hovered { hover_color } else { ui.visuals().widgets.noninteractive.bg_stroke.color };
    let fg = if hovered { hover_color } else { color };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(m.rounding),
        fill,
        egui::Stroke::new(1.0_f32, border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        egui::FontId::proportional(m.font_icon),
        fg,
    );
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A `color`-outlined button with the label in that color and a faint tint on
/// hover.
pub fn outlined_button(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    label: &str,
    color: egui::Color32,
) -> egui::Response {
    let base = ui.visuals().panel_fill;
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());
    let fill = if resp.hovered() {
        crate::style::mix(color, base, 0.85)
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(crate::style::METRICS.rounding),
        fill,
        egui::Stroke::new(1.0_f32, crate::style::mix(color, base, 0.35)),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(crate::style::METRICS.font_label),
        color,
    );
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn style_muted_hscroll(ui: &mut egui::Ui) {
    let base = ui.visuals().panel_fill;
    let text = ui.visuals().text_color();
    let handle = mix(base, text, 0.16);
    let handle_active = mix(base, text, 0.30);
    let style = ui.style_mut();
    style.always_scroll_the_only_direction = true;
    style.spacing.scroll.floating = false;
    style.spacing.scroll.bar_width = crate::style::METRICS.gap_small;
    style.spacing.scroll.foreground_color = false;
    style.visuals.widgets.inactive.bg_fill = handle;
    style.visuals.widgets.hovered.bg_fill = handle_active;
    style.visuals.widgets.active.bg_fill = handle_active;
}

pub fn panel_header(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let border = ui.visuals().widgets.noninteractive.bg_stroke.color;
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(width, crate::style::METRICS.header_height),
        egui::Sense::hover(),
    );
    let pad = crate::style::METRICS.padding;
    let content = egui::Rect::from_min_max(
        egui::pos2(rect.left() + pad, rect.top()),
        egui::pos2(rect.right() - pad, rect.bottom()),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    add_contents(&mut child);
    ui.painter().hline(rect.x_range(), rect.bottom() - 0.5, egui::Stroke::new(1.0_f32, border));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn empty_label(ui: &mut egui::Ui) {
    ui.add(egui::Label::new(egui::RichText::new("Пусто...").weak()).selectable(false));
}
