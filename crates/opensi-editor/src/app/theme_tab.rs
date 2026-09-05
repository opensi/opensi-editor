use opensi_core::prelude::*;

use crate::{
    app::context::ThemeContext,
    element::{
        dashed_add_row, icon_button,
        node_context::PackageNodeContextMenu,
        plural, price_range_label,
        props::{chips_field, meta_row, section_label, text_field},
        question_is_filled,
    },
    icon,
    style::METRICS,
};

/// Theme editor: a vertical list of question rows .
pub fn theme_tab(ctx: &mut ThemeContext, ui: &mut egui::Ui) {
    let theme_idx = ctx.idx();
    let selected = ctx.selected();
    let count = ctx.package().count_questions(theme_idx);
    let width = ui.available_width();
    ui.spacing_mut().item_spacing.y = METRICS.padding_small;

    for i in 0..count {
        let q_idx = theme_idx.question(i);
        let Some((price, filled, text, meta)) = ctx
            .package()
            .get_question(q_idx)
            .map(|q| (q.price, question_is_filled(q), question_text(q), question_meta(q)))
        else {
            continue;
        };
        let sel = matches!(selected, Some(PackageNode::Question(x)) if x == q_idx);

        let row = question_row(ui, width, price, filled, &text, &meta, sel);
        if row.duplicate {
            ctx.package().duplicate_node(q_idx.into());
        } else if row.delete {
            ctx.package().remove_node(q_idx.into());
        } else if row.select {
            ctx.select(q_idx.into());
        }
        PackageNodeContextMenu { package: ctx.package(), node: q_idx.into() }
            .show(&row.response, ui);
    }

    if dashed_add_row(ui, "+ Добавить вопрос", false).clicked() {
        ctx.package().allocate_question(theme_idx);
    }
    ui.add_space(METRICS.gap_tiny);
}

fn question_text(question: &Question) -> String {
    question
        .scenario
        .iter()
        .find(|atom| atom.kind.is_text() && !atom.body.is_empty())
        .map(|atom| atom.body.clone())
        .unwrap_or_else(|| {
            if question_is_filled(question) {
                "Медиа-вопрос".to_string()
            } else {
                "Пустой вопрос".to_string()
            }
        })
}

fn question_meta(question: &Question) -> String {
    if !question_is_filled(question) {
        return "нет сценария и ответов".to_string();
    }
    let atoms = question.scenario.len();
    let answers = question.right.len();
    format!(
        "{atoms} {} · {answers} {}",
        plural(atoms, "фрагмент", "фрагмента", "фрагментов"),
        plural(answers, "ответ", "ответа", "ответов"),
    )
}

struct RowResp {
    response: egui::Response,
    select: bool,
    duplicate: bool,
    delete: bool,
}

fn question_row(
    ui: &mut egui::Ui,
    width: f32,
    price: usize,
    filled: bool,
    text: &str,
    meta: &str,
    selected: bool,
) -> RowResp {
    let visuals = ui.visuals();
    let accent = visuals.selection.stroke.color;
    let sel_fill = visuals.selection.bg_fill;
    let raised = visuals.window_fill;
    let hover_fill = visuals.widgets.hovered.bg_fill;
    let border = visuals.widgets.noninteractive.bg_stroke.color;
    let border_strong = visuals.widgets.hovered.bg_stroke.color;
    let strong = visuals.strong_text_color();
    let text_color = visuals.text_color();
    let weak = visuals.weak_text_color();
    let danger = visuals.error_fg_color;

    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, METRICS.list_row_height), egui::Sense::click());
    let hovered = response.hovered();
    let cy = rect.center().y;

    let fill = if selected {
        sel_fill
    } else if hovered {
        hover_fill
    } else {
        raised
    };
    let edge = if selected || hovered { border_strong } else { border };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        fill,
        egui::Stroke::new(1.0, edge),
        egui::StrokeKind::Inside,
    );
    let bar_color = if filled { accent } else { border };
    let bar =
        egui::Rect::from_min_size(rect.min, egui::vec2(METRICS.accent_bar_width, rect.height()));
    ui.painter().rect_filled(bar, egui::CornerRadius { nw: 4, sw: 4, ne: 0, se: 0 }, bar_color);

    let price_color = if filled { strong } else { weak };
    ui.painter().text(
        egui::pos2(rect.left() + METRICS.padding, cy),
        egui::Align2::LEFT_CENTER,
        price.to_string(),
        egui::FontId::proportional(METRICS.font_large),
        price_color,
    );

    let btn = egui::Vec2::splat(METRICS.compact_size);
    let actions_rect = egui::Rect::from_min_max(
        egui::pos2(rect.right() - METRICS.padding - 2.0 * btn.x - METRICS.gap_tiny, rect.top()),
        egui::pos2(rect.right() - METRICS.padding, rect.bottom()),
    );
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(actions_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = METRICS.gap_tiny;
    let del =
        icon_button(&mut actions, btn, icon!(TRASH), weak, danger, egui::Color32::TRANSPARENT);
    let dup = icon_button(&mut actions, btn, icon!(COPY), weak, strong, egui::Color32::TRANSPARENT);

    let text_left = rect.left() + METRICS.padding + METRICS.grid_cell_width + METRICS.padding;
    let text_rect = egui::Rect::from_min_max(
        egui::pos2(text_left, rect.top() + METRICS.padding_small),
        egui::pos2(
            actions_rect.left() - METRICS.padding_small,
            rect.bottom() - METRICS.padding_small,
        ),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new().max_rect(text_rect).layout(egui::Layout::top_down(egui::Align::Min)),
    );
    child.spacing_mut().item_spacing.y = METRICS.gap_tiny;
    let title_color = if filled { text_color } else { weak };
    child.add(
        egui::Label::new(egui::RichText::new(text).size(METRICS.font_body).color(title_color))
            .truncate()
            .selectable(false),
    );
    child.add(
        egui::Label::new(egui::RichText::new(meta).size(METRICS.font_small).color(weak))
            .truncate()
            .selectable(false),
    );

    let select = response.clicked() && !dup.clicked() && !del.clicked();
    RowResp { response, select, duplicate: dup.clicked(), delete: del.clicked() }
}

pub fn theme_properties(ctx: &mut ThemeContext, ui: &mut egui::Ui) {
    let (filled, total) = {
        let theme = ctx.theme();
        (theme.questions.iter().filter(|q| question_is_filled(q)).count(), theme.questions.len())
    };

    let theme = ctx.theme();
    let range = price_range_label(theme.questions.iter().map(|q| q.price));

    text_field(ui, "Название", &mut theme.name, "Название темы");

    let info = theme.info.get_or_insert_with(Default::default);
    text_field(ui, "Комментарий ведущего", &mut info.comments, "Не указан");
    text_field(ui, "Расширения", &mut info.extension, "");
    chips_field(ui, "Авторы", "+ Добавить автора", "theme-authors", &mut info.authors);
    chips_field(ui, "Источники", "+ Добавить источник", "theme-sources", &mut info.sources);

    ui.add_space(METRICS.gap_small);
    ui.separator();
    ui.add_space(METRICS.padding);
    section_label(ui, "Сводка");
    ui.add_space(METRICS.padding_small);
    meta_row(ui, "Вопросы", &total.to_string());
    meta_row(ui, "Заполнено", &format!("{filled} / {total}"));
    meta_row(ui, "Диапазон", &range);
}
