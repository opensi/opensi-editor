use opensi_core::prelude::*;

use crate::{
    app::{FONT_BOLD_ID, context::RoundContext},
    element::{
        dashed_add_row, dashed_button,
        node_context::PackageNodeContextMenu,
        price_range_label,
        props::{chips_field, field_block, field_label, meta_row, section_label, text_field},
        question_is_filled, style_muted_hscroll,
    },
    style::{self, ColorScheme, METRICS, mix},
};

/// Workarea tab: the round overview.
pub fn round_tab(ctx: &mut RoundContext, ui: &mut egui::Ui) {
    let scheme = style::current_scheme(ui.ctx());
    let round_idx = ctx.idx();
    let selected = ctx.selected();
    let theme_count = ctx.package().count_themes(round_idx);
    {
        let avail = ui.available_width();
        let theme_w = METRICS.grid_theme_width.min(avail * 0.5);
        let add_w = METRICS.grid_add_width;
        let cell_w = METRICS.grid_cell_width;

        ui.scope(|ui| {
            style_muted_hscroll(ui);

            egui::ScrollArea::horizontal()
                .id_salt(("round-grid", round_idx.index))
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = METRICS.padding_small;

                    for t in 0..theme_count {
                        let theme_idx = round_idx.theme(t);
                        let name = ctx
                            .package()
                            .get_theme(theme_idx)
                            .map(|theme| theme.name.clone())
                            .unwrap_or_default();
                        let question_count = ctx.package().count_questions(theme_idx);

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(METRICS.gap, 0.0);

                            let sel =
                                matches!(selected, Some(PackageNode::Theme(i)) if i == theme_idx);
                            let resp =
                                theme_cell(ui, egui::vec2(theme_w, METRICS.grid_row_height), &name, sel, &scheme);
                            if resp.clicked() {
                                ctx.select(theme_idx.into());
                            }
                            PackageNodeContextMenu { package: ctx.package(), node: theme_idx.into() }
                                .show(&resp, ui);

                            for qi in 0..question_count {
                                let q_idx = theme_idx.question(qi);
                                let (price, filled) = ctx
                                    .package()
                                    .get_question(q_idx)
                                    .map(|q| (q.price, question_is_filled(q)))
                                    .unwrap_or((0, false));
                                let sel =
                                    matches!(selected, Some(PackageNode::Question(i)) if i == q_idx);
                                let resp = question_cell(
                                    ui,
                                    egui::vec2(cell_w, METRICS.grid_row_height),
                                    price,
                                    filled,
                                    sel,
                                    &scheme,
                                );
                                if resp.clicked() {
                                    ctx.select(q_idx.into());
                                }
                                PackageNodeContextMenu { package: ctx.package(), node: q_idx.into() }
                                    .show(&resp, ui);
                            }

                            if dashed_button(ui, egui::vec2(add_w, METRICS.grid_row_height), "+ Вопрос", 12.5, None)
                                .clicked()
                            {
                                ctx.package().allocate_question(theme_idx);
                            }
                        });
                    }
                });
        });

        ui.add_space(METRICS.padding_small);
        if dashed_add_row(ui, "+ Добавить тему", false).clicked() {
            ctx.package().allocate_theme(round_idx);
        }
        ui.add_space(METRICS.gap);
    }
}

pub fn round_properties(ctx: &mut RoundContext, ui: &mut egui::Ui) {
    let round = ctx.round();

    // Derived summary values, gathered before the info fields borrow.
    let theme_count = round.themes.len();
    let total: usize = round.themes.iter().map(|theme| theme.questions.len()).sum();
    let filled: usize = round
        .themes
        .iter()
        .flat_map(|theme| &theme.questions)
        .filter(|&q| question_is_filled(q))
        .count();
    let range = price_range_label(
        round.themes.iter().flat_map(|theme| theme.questions.iter().map(|q| q.price)),
    );

    text_field(ui, "Название", &mut round.name, "Название раунда");
    field_block(ui, |ui| {
        field_label(ui, "Тип раунда");
        round_type_segmented(round, ui);
    });

    let info = round.info.get_or_insert_with(Default::default);
    text_field(ui, "Комментарий", &mut info.comments, "Заметка к раунду");
    text_field(ui, "Расширения", &mut info.extension, "");
    chips_field(ui, "Авторы", "+ Добавить автора", "round-authors", &mut info.authors);
    chips_field(ui, "Источники", "+ Добавить источник", "round-sources", &mut info.sources);

    ui.add_space(METRICS.gap_small);
    ui.separator();
    ui.add_space(METRICS.padding);
    section_label(ui, "Сводка");
    ui.add_space(METRICS.padding_small);
    meta_row(ui, "Темы", &theme_count.to_string());
    meta_row(ui, "Вопросы", &format!("{filled} / {total}"));
    meta_row(ui, "Диапазон", &range);
}

fn round_type_segmented(round: &mut Round, ui: &mut egui::Ui) {
    const OPTIONS: [(&str, Option<&str>); 3] =
        [("Обычный", None), ("Финал", Some("final")), ("Аукцион", Some("auction"))];
    let selected = OPTIONS.iter().position(|(_, kind)| *kind == round.kind.as_deref());

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = METRICS.gap_small;
        let n = OPTIONS.len();
        let w = (ui.available_width() - METRICS.gap_small * (n as f32 - 1.0)) / n as f32;
        for (i, (label, kind)) in OPTIONS.iter().enumerate() {
            if segmented_button(ui, w, label, selected == Some(i)).clicked() {
                round.kind = kind.map(str::to_string);
            }
        }
    });
}

fn segmented_button(ui: &mut egui::Ui, width: f32, label: &str, selected: bool) -> egui::Response {
    let accent = ui.visuals().selection.stroke.color;
    let sel_fill = ui.visuals().selection.bg_fill;
    let border = ui.visuals().widgets.inactive.bg_stroke.color;
    let border_strong = ui.visuals().widgets.hovered.bg_stroke.color;
    let hover_fill = ui.visuals().widgets.hovered.bg_fill;
    let text = ui.visuals().text_color();
    let weak = ui.visuals().weak_text_color();
    let (rect, resp) =
        ui.allocate_exact_size(egui::vec2(width, METRICS.button_height), egui::Sense::click());
    let hovered = resp.hovered();
    let (fill, bd, fg) = if selected {
        (sel_fill, accent, accent)
    } else if hovered {
        (hover_fill, border_strong, text)
    } else {
        (egui::Color32::TRANSPARENT, border, weak)
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        fill,
        egui::Stroke::new(1.0_f32, bd),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(METRICS.font_label),
        fg,
    );
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn theme_cell(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    name: &str,
    selected: bool,
    sc: &ColorScheme,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());
    let hovered = resp.hovered();
    let fill = if selected {
        mix(sc.base, sc.accent, 0.14)
    } else if hovered {
        sc.hover_fill()
    } else {
        raised(sc)
    };
    let border = if selected {
        sc.accent
    } else if hovered {
        sc.border_strong()
    } else {
        sc.border()
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        fill,
        egui::Stroke::new(1.0_f32, border),
        egui::StrokeKind::Inside,
    );
    let bar =
        egui::Rect::from_min_size(rect.min, egui::vec2(METRICS.accent_bar_width, rect.height()));
    ui.painter().rect_filled(bar, egui::CornerRadius { nw: 4, sw: 4, ne: 0, se: 0 }, sc.accent);

    let text_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + METRICS.padding, rect.top()),
        egui::pos2(rect.right() - METRICS.padding_small, rect.bottom()),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(text_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.add(
        egui::Label::new(
            egui::RichText::new(name)
                .size(METRICS.font_body)
                .color(sc.text_strong)
                .family(egui::FontFamily::Name(FONT_BOLD_ID.into())),
        )
        .truncate()
        .selectable(false),
    );
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn question_cell(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    price: usize,
    filled: bool,
    selected: bool,
    sc: &ColorScheme,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());
    let hovered = resp.hovered();
    let fill = if selected {
        mix(sc.base, sc.accent, 0.14)
    } else if hovered {
        sc.hover_fill()
    } else if filled {
        raised(sc)
    } else {
        sc.base_weak
    };
    let border = if selected {
        sc.accent
    } else if hovered {
        sc.border_strong()
    } else if filled {
        sc.border()
    } else {
        mix(sc.base_weak, sc.text_weak, 0.14)
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        fill,
        egui::Stroke::new(1.0_f32, border),
        egui::StrokeKind::Inside,
    );

    let price_color = if filled { sc.text_strong } else { sc.text_weak };
    ui.painter().text(
        egui::pos2(rect.center().x, rect.top() + rect.height() * 0.42),
        egui::Align2::CENTER_CENTER,
        price.to_string(),
        egui::FontId::new(METRICS.font_large, egui::FontFamily::Name(FONT_BOLD_ID.into())),
        price_color,
    );
    let bar_color = if filled { sc.accent } else { mix(fill, sc.text_weak, 0.4) };
    let bar = egui::Rect::from_center_size(
        egui::pos2(rect.center().x, rect.bottom() - METRICS.padding),
        egui::vec2((rect.width() - 2.0 * METRICS.padding_small).max(0.0), METRICS.accent_bar_width),
    );
    ui.painter().rect_filled(bar, egui::CornerRadius::same(METRICS.hairline as u8), bar_color);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn raised(sc: &ColorScheme) -> egui::Color32 {
    mix(sc.base, sc.text, 0.02)
}
