use opensi_core::prelude::*;

use crate::{
    app::context::PackageContext,
    element::{
        dashed_add_row, dashed_chip, plural, price_range,
        props::{
            chips_field, compact_input, field_block, field_label, meta_row, section_label,
            text_field,
        },
        style_muted_hscroll,
    },
    style::{METRICS, mix},
};

/// Workarea tab: the package overview (rounds with their themes).
pub fn package_tab(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    ui.spacing_mut().item_spacing.y = METRICS.padding_small;
    let round_count = ctx.package().count_rounds();
    for r in 0..round_count {
        round_card(ctx, r.into(), ui);
    }
    if dashed_add_row(ui, "+ Добавить раунд", true).clicked() {
        ctx.package().allocate_round();
    }
}

fn round_meta(round: &Round) -> String {
    let theme_count = round.themes.len();
    let question_count: usize = round.themes.iter().map(|t| t.questions.len()).sum();
    let mut meta = format!(
        "{} {} · {} {}",
        theme_count,
        plural(theme_count, "тема", "темы", "тем"),
        question_count,
        plural(question_count, "вопрос", "вопроса", "вопросов"),
    );
    let all_prices = round.themes.iter().flat_map(|t| t.questions.iter().map(|q| q.price));
    if let Some((min, max)) = price_range(all_prices) {
        meta.push_str(&format!(" · {min}–{max}"));
    }
    meta
}

fn round_card(ctx: &mut PackageContext, idx: RoundIdx, ui: &mut egui::Ui) {
    let Some(round) = ctx.package().get_round(idx) else { return };
    let name = round.name.clone();
    let meta = round_meta(round);
    let theme_count = round.themes.len();

    let visuals = ui.visuals();
    let accent = visuals.selection.stroke.color;
    let fill = visuals.widgets.inactive.bg_fill;
    let border = visuals.widgets.inactive.bg_stroke;
    let title = visuals.text_color();
    let strong = visuals.strong_text_color();
    let weak = visuals.weak_text_color();

    let inner = egui::Frame::new()
        .fill(fill)
        .stroke(border)
        .corner_radius(0)
        .inner_margin(METRICS.card_margin())
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let title_galley = ui.fonts(|f| {
                    f.layout_no_wrap(
                        name.clone(),
                        egui::FontId::proportional(METRICS.font_heading),
                        egui::Color32::PLACEHOLDER,
                    )
                });
                let (title_rect, title_resp) =
                    ui.allocate_exact_size(title_galley.size(), egui::Sense::click());
                let title_color = if title_resp.hovered() { strong } else { title };
                ui.painter().galley(title_rect.min, title_galley, title_color);
                if title_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                    ctx.select(idx.into());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&meta).size(METRICS.font_small).color(weak),
                        )
                        .selectable(false),
                    );
                });
            });
            ui.add_space(METRICS.padding);
            ui.scope(|ui| {
                style_muted_hscroll(ui);
                egui::ScrollArea::horizontal()
                    .id_salt(("round-themes", idx.index))
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = METRICS.gap;
                            for t in 0..theme_count {
                                let theme_idx = idx.theme(t);
                                let Some((name, range)) =
                                    ctx.package().get_theme(theme_idx).map(|theme| {
                                        (
                                            theme.name.clone(),
                                            price_range(theme.questions.iter().map(|q| q.price)),
                                        )
                                    })
                                else {
                                    continue;
                                };
                                if theme_chip(ui, &name, range).clicked() {
                                    ctx.select(theme_idx.into());
                                }
                            }
                            if dashed_chip(ui, "+ Тема").clicked() {
                                ctx.package().allocate_theme(idx);
                            }
                        });
                    });
            });
        });

    let rect = inner.response.rect;
    let bar = egui::Rect::from_min_size(
        rect.left_top(),
        egui::vec2(METRICS.accent_bar_width, rect.height()),
    );
    ui.painter().rect_filled(bar, egui::CornerRadius::ZERO, accent);
}

fn theme_chip(ui: &mut egui::Ui, name: &str, range: Option<(usize, usize)>) -> egui::Response {
    let base = ui.visuals().panel_fill;
    let border = ui.visuals().widgets.inactive.bg_stroke.color;
    let border_hover = ui.visuals().widgets.hovered.bg_stroke.color;
    let hover_fill = mix(base, ui.visuals().text_color(), 0.05);
    let text_color = ui.visuals().text_color();
    let weak = ui.visuals().weak_text_color();

    // Measure the content first so the hover fill/border apply the same frame.
    let name_galley = ui.fonts(|f| {
        f.layout_no_wrap(name.to_owned(), egui::FontId::proportional(METRICS.font_body), text_color)
    });
    let range_galley = range.map(|(min, max)| {
        ui.fonts(|f| {
            f.layout_no_wrap(
                format!("{min}–{max}"),
                egui::FontId::proportional(METRICS.font_small),
                weak,
            )
        })
    });

    let (pad_x, pad_y, gap) = (METRICS.padding_small, METRICS.gap, METRICS.gap);
    let name_size = name_galley.size();
    let range_size = range_galley.as_ref().map(|g| g.size());
    let content_w = name_size.x + range_size.map_or(0.0, |s| gap + s.x);
    let content_h = name_size.y.max(range_size.map_or(0.0, |s| s.y));
    let size = egui::vec2(content_w + pad_x * 2.0, content_h + pad_y * 2.0);

    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let hovered = response.hovered();
    let fill = if hovered { hover_fill } else { base };
    let edge = if hovered { border_hover } else { border };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        fill,
        egui::Stroke::new(1.0_f32, edge),
        egui::StrokeKind::Inside,
    );

    let cy = rect.center().y;
    ui.painter().galley(
        egui::pos2(rect.left() + pad_x, cy - name_size.y / 2.0),
        name_galley,
        text_color,
    );
    if let (Some(g), Some(rs)) = (range_galley, range_size) {
        ui.painter().galley(
            egui::pos2(rect.left() + pad_x + name_size.x + gap, cy - rs.y / 2.0),
            g,
            weak,
        );
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn package_properties(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    let package = ctx.package();

    text_field(ui, "Название", &mut package.name, "Название пакета");
    field_block(ui, |ui| {
        field_label(ui, "Сложность");
        let mut difficulty = package.difficulty.to_string();
        if compact_input(ui, &mut difficulty, "1–10").changed() {
            if let Ok(value) = difficulty.parse::<u8>() {
                package.difficulty = value.min(10);
            }
        }
    });
    text_field(ui, "Ограничения", &mut package.restriction, "Не указаны");
    text_field(ui, "Дата создания", &mut package.date, "");
    text_field(ui, "Издатель", &mut package.publisher, "Не указан");
    text_field(ui, "Язык", &mut package.language, "ru-RU");
    text_field(ui, "Комментарий", &mut package.info.comments, "Заметка к пакету");
    text_field(ui, "Расширения", &mut package.info.extension, "");

    chips_field(ui, "Тэги", "+ тэг", "tags", &mut package.tags);
    chips_field(ui, "Авторы", "+ Добавить автора", "authors", &mut package.info.authors);
    chips_field(ui, "Источники", "+ Добавить источник", "sources", &mut package.info.sources);

    ui.add_space(METRICS.gap_small);
    ui.separator();
    ui.add_space(METRICS.padding);
    section_label(ui, "Метаданные");
    ui.add_space(METRICS.padding_small);
    meta_row(ui, "ID пакета", &package.id);
    meta_row(ui, "Версия пакета", &format!("{:.1}", package.version));
}
