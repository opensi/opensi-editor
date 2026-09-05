use crate::{element::common::dashed_border, style::METRICS};

/// An uppercase section header row: the title in `color`, a dim note on the right.
pub fn section_head(ui: &mut egui::Ui, title: &str, color: egui::Color32, trailing: &str) {
    let m = &METRICS;
    let dim = crate::style::mix(ui.visuals().weak_text_color(), ui.visuals().panel_fill, 0.4);
    ui.horizontal(|ui| {
        ui.add(
            egui::Label::new(
                egui::RichText::new(title.to_uppercase()).size(m.font_small).strong().color(color),
            )
            .selectable(false),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(
                egui::Label::new(egui::RichText::new(trailing).size(m.font_small).color(dim))
                    .selectable(false),
            );
        });
    });
}

/// Uppercase, muted section label (panel header and metadata title).
pub fn section_label(ui: &mut egui::Ui, label: &str) {
    ui.add(
        egui::Label::new(
            egui::RichText::new(label.to_uppercase())
                .size(METRICS.font_small)
                .strong()
                .color(ui.visuals().weak_text_color()),
        )
        .selectable(false),
    );
}

/// A label + control stacked with a 6px gap, then a 14px gap to the next field.
pub fn field_block(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = METRICS.gap_small;
        add_contents(ui);
    });
    ui.add_space(METRICS.padding);
}

pub fn field_label(ui: &mut egui::Ui, label: &str) {
    ui.add(
        egui::Label::new(
            egui::RichText::new(label)
                .size(METRICS.font_label)
                .color(ui.visuals().weak_text_color()),
        )
        .selectable(false),
    );
}

/// A compact, full-width single-line text input matching the design's fields.
pub fn compact_input(ui: &mut egui::Ui, value: &mut String, hint: &str) -> egui::Response {
    let mut edit = egui::TextEdit::singleline(value)
        .desired_width(f32::INFINITY)
        .margin(egui::Margin::symmetric(METRICS.padding_small as i8, METRICS.gap as i8))
        .font(egui::FontId::proportional(METRICS.font_body));
    if !hint.is_empty() {
        edit = edit.hint_text(hint);
    }
    ui.add(edit)
}

pub fn text_field(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    field_block(ui, |ui| {
        field_label(ui, label);
        compact_input(ui, value, hint);
    });
}

/// A wrapping row of removable chips plus a dashed "+ ..." add field.
pub fn chips_field(
    ui: &mut egui::Ui,
    label: &str,
    add_hint: &str,
    id: &str,
    items: &mut Vec<String>,
) {
    field_block(ui, |ui| {
        field_label(ui, label);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::splat(METRICS.gap_small);
            let mut removed = None;
            for (index, item) in items.iter().enumerate() {
                if chip(ui, item, id, index) {
                    removed = Some(index);
                }
            }
            if let Some(index) = removed {
                items.remove(index);
            }
            dashed_add(ui, items, id, add_hint);
        });
    });
}

/// A single tag chip; returns `true` when its × was clicked.
fn chip(ui: &mut egui::Ui, text: &str, id: &str, index: usize) -> bool {
    let chip_text = ui.visuals().selection.stroke.color;
    let chip_bg = ui.visuals().widgets.inactive.bg_fill;
    let chip_border = ui.visuals().widgets.inactive.bg_stroke.color;
    let chip_x = ui.visuals().weak_text_color();
    let font = egui::FontId::proportional(METRICS.font_label);
    let galley = ui.fonts(|f| f.layout_no_wrap(text.to_owned(), font.clone(), chip_text));
    let pad = egui::vec2(METRICS.gap, METRICS.gap_small);
    let gap = METRICS.gap_small;
    let x_w = METRICS.font_small;
    let size = egui::vec2(
        pad.x * 2.0 + galley.size().x + gap + x_w,
        pad.y * 2.0 + galley.size().y.max(x_w),
    );
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        chip_bg,
        egui::Stroke::new(1.0_f32, chip_border),
        egui::StrokeKind::Inside,
    );
    let text_pos = egui::pos2(rect.left() + pad.x, rect.center().y - galley.size().y / 2.0);
    ui.painter().galley(text_pos, galley, chip_text);
    let x_center = egui::pos2(rect.right() - pad.x - x_w / 2.0, rect.center().y);
    let x_rect =
        egui::Rect::from_center_size(x_center, egui::vec2(x_w + METRICS.gap_small, rect.height()));
    let x_response =
        ui.interact(x_rect, egui::Id::new((id, "chip-x", index)), egui::Sense::click());
    let x_color = if x_response.hovered() { ui.visuals().text_color() } else { chip_x };
    ui.painter().text(x_center, egui::Align2::CENTER_CENTER, "×", font, x_color);
    x_response.clicked()
}

/// An inline dashed "+ ..." field: type and press Enter to append an item. The
/// placeholder is painted with the design's brighter dashed-text color.
fn dashed_add(ui: &mut egui::Ui, items: &mut Vec<String>, id: &str, hint: &str) {
    let state_id = egui::Id::new((id, "chip-add"));
    let dash_text = ui.visuals().weak_text_color();
    let dash_border = ui.visuals().widgets.noninteractive.bg_stroke.color;
    let mut text: String = ui.data(|d| d.get_temp(state_id).unwrap_or_default());
    let font = egui::FontId::proportional(METRICS.font_label);
    let sample = if text.is_empty() { hint.to_owned() } else { text.clone() };
    let sample_w = ui.fonts(|f| f.layout_no_wrap(sample, font.clone(), dash_text)).size().x;
    let (width, height) =
        ((sample_w + 2.0 * METRICS.padding).max(2.0 * METRICS.compact_size), METRICS.compact_size);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let inner = rect.shrink2(egui::vec2(METRICS.padding_small, METRICS.gap_tiny));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inner)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let response = child.add(
        egui::TextEdit::singleline(&mut text)
            .frame(false)
            .desired_width(f32::INFINITY)
            .font(font.clone()),
    );
    let focused = response.has_focus();
    let edge = if focused { ui.visuals().selection.stroke.color } else { dash_border };
    dashed_border(ui.painter(), rect, edge);
    if text.is_empty() && !focused {
        ui.painter().text(
            egui::pos2(inner.left(), rect.center().y),
            egui::Align2::LEFT_CENTER,
            hint,
            font,
            dash_text,
        );
    }
    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            items.push(trimmed.to_owned());
            text.clear();
        }
    }
    ui.data_mut(|d| d.insert_temp(state_id, text));
}

pub fn meta_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = METRICS.gap_tiny;
        field_label(ui, label);
        ui.add(
            egui::Label::new(
                egui::RichText::new(value)
                    .monospace()
                    .size(METRICS.font_small)
                    .color(ui.visuals().weak_text_color()),
            )
            .truncate(),
        );
    });
    ui.add_space(METRICS.padding_small);
}
