//! The "Editor" landing page shown when no package is open.

use crate::{
    app::context::AppContext,
    icon,
    style::{METRICS, mix},
};

/// The landing page.
pub fn welcome_page(ctx: &mut AppContext, ui: &mut egui::Ui) {
    let accent = ui.visuals().selection.stroke.color;
    let strong = ui.visuals().strong_text_color();
    let weak = ui.visuals().weak_text_color();

    ui.vertical_centered(|ui| {
        let top_space =
            ((ui.available_height() - METRICS.hero_height) / 2.0).max(METRICS.padding_small);
        ui.add_space(top_space);

        // Logo badge: a fixed-size rounded square painted directly so it never
        // stretches to the panel width.
        let badge_size = METRICS.hero_badge_size;
        let (badge_rect, _) =
            ui.allocate_exact_size(egui::vec2(badge_size, badge_size), egui::Sense::hover());
        ui.painter().rect_filled(
            badge_rect,
            egui::CornerRadius::same((METRICS.hero_badge_size / 4.0) as u8),
            accent,
        );
        ui.painter().text(
            badge_rect.center(),
            egui::Align2::CENTER_CENTER,
            icon!(GRADUATION_CAP),
            egui::FontId::proportional(METRICS.font_display),
            mix(accent, egui::Color32::BLACK, 0.82),
        );

        ui.add_space(METRICS.padding);
        ui.add(
            egui::Label::new(
                egui::RichText::new("OpenSI Editor").size(METRICS.font_display).color(strong),
            )
            .selectable(false),
        );

        ui.add_space(METRICS.gap_small);
        ui.add(
            egui::Label::new(
                egui::RichText::new(
                    "Пакет не открыт. Создайте новый или продолжите работу над существующим.",
                )
                .size(METRICS.font_medium)
                .color(weak),
            )
            .selectable(false),
        );

        ui.add_space(METRICS.padding + METRICS.gap);

        let new_icon = icon!(FOLDER_SIMPLE_PLUS);
        let open_icon = icon!(FOLDER_OPEN);
        let w_new = welcome_button_width(ui, new_icon, "Новый пакет");
        let w_open = welcome_button_width(ui, open_icon, "Открыть пакет");
        let gap = METRICS.gap;
        ui.allocate_ui_with_layout(
            egui::vec2(w_new + gap + w_open, METRICS.add_row_height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = gap;
                if welcome_button(ui, new_icon, "Новый пакет", true, w_new).clicked() {
                    ctx.new_package();
                }
                if welcome_button(ui, open_icon, "Открыть пакет", false, w_open).clicked()
                {
                    ctx.pick_new_package();
                }
            },
        );
    });
}

fn welcome_button_width(ui: &egui::Ui, icon: &str, label: &str) -> f32 {
    let galley = ui.fonts(|f| {
        f.layout_no_wrap(
            format!("{}  {}", icon, label),
            egui::FontId::proportional(METRICS.font_icon),
            egui::Color32::WHITE,
        )
    });
    galley.size().x + METRICS.card_padding * 2.0
}

fn welcome_button(
    ui: &mut egui::Ui,
    icon: &str,
    label: &str,
    primary: bool,
    width: f32,
) -> egui::Response {
    ui.scope(|ui| {
        let visuals = ui.visuals();
        let accent = visuals.selection.stroke.color;
        let text = visuals.text_color();
        let base_weak = visuals.faint_bg_color;
        let border = visuals.widgets.noninteractive.bg_stroke.color;

        // On the accent-filled primary button, use a dark tint of the accent
        // for contrast.
        let text_color = if primary { mix(accent, egui::Color32::BLACK, 0.82) } else { text };
        let (idle, hovered, pressed, strokes) = if primary {
            (
                accent,
                mix(accent, egui::Color32::WHITE, 0.12),
                mix(accent, egui::Color32::BLACK, 0.10),
                [egui::Stroke::NONE; 3],
            )
        } else {
            (
                base_weak,
                mix(base_weak, text, 0.08),
                mix(base_weak, text, 0.14),
                [
                    egui::Stroke::new(1.0, border),
                    egui::Stroke::new(1.0, mix(border, text, 0.30)),
                    egui::Stroke::new(1.0, mix(border, text, 0.35)),
                ],
            )
        };

        let w = &mut ui.visuals_mut().widgets;
        for (wv, (fill, stroke)) in [&mut w.inactive, &mut w.hovered, &mut w.active]
            .into_iter()
            .zip([idle, hovered, pressed].into_iter().zip(strokes))
        {
            wv.weak_bg_fill = fill;
            wv.bg_fill = fill;
            wv.bg_stroke = stroke;
            wv.fg_stroke.color = text_color;
            wv.corner_radius = egui::CornerRadius::same(METRICS.rounding_large);
        }

        let label = egui::RichText::new(format!("{}  {}", icon, label))
            .size(METRICS.font_icon)
            .color(text_color);
        ui.add_sized(egui::vec2(width, METRICS.add_row_height), egui::Button::new(label))
            .on_hover_cursor(egui::CursorIcon::PointingHand)
    })
    .inner
}
