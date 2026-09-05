use opensi_core::prelude::*;

use crate::{
    app::context::QuestionContext,
    element::{
        dashed_button, icon_button, outlined_button, plural,
        props::{
            chips_field, compact_input, field_block, field_label, meta_row, section_head,
            section_label, text_field,
        },
    },
    icon,
    style::{METRICS, mix},
};

pub fn question_tab(ctx: &mut QuestionContext, ui: &mut egui::Ui) {
    let package_id = ctx.package().id.clone();
    let q_idx = ctx.idx();
    let mut pick_image = false;

    ui.spacing_mut().item_spacing.y = METRICS.padding;
    let question = ctx.question();
    scenario_section(question, &package_id, &mut pick_image, ui);
    answers_section(question, ui);

    if pick_image {
        ctx.pick_new_image_for(q_idx);
    }
}

fn scenario_section(
    question: &mut Question,
    package_id: &str,
    pick_image: &mut bool,
    ui: &mut egui::Ui,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = METRICS.padding_small;
        let n = question.scenario.len();
        let weak = ui.visuals().weak_text_color();
        section_head(
            ui,
            "Сценарий",
            weak,
            &format!("{n} {}", plural(n, "фрагмент", "фрагмента", "фрагментов")),
        );

        let mut remove = None;
        for (i, atom) in question.scenario.iter_mut().enumerate() {
            if atom_card(atom, package_id, ui) {
                remove = Some(i);
            }
        }
        if let Some(i) = remove {
            question.scenario.remove(i);
        }

        ui.horizontal(|ui| {
            let gap = METRICS.gap_small;
            ui.spacing_mut().item_spacing.x = gap;
            let size = egui::vec2((ui.available_width() - gap * 3.0) / 4.0, METRICS.button_height);
            if dashed_button(ui, size, "+ Текст", METRICS.font_label, None).clicked() {
                question.scenario.push(Atom { kind: AtomKind::Text, ..Atom::default() });
            }
            if dashed_button(ui, size, "+ Изображение", METRICS.font_label, None).clicked()
            {
                *pick_image = true;
            }
            if dashed_button(ui, size, "+ Аудио", METRICS.font_label, None).clicked() {
                question.scenario.push(Atom { kind: AtomKind::Voice, ..Atom::default() });
            }
            if dashed_button(ui, size, "+ Видео", METRICS.font_label, None).clicked() {
                question.scenario.push(Atom { kind: AtomKind::Video, ..Atom::default() });
            }
        });
    });
}

fn atom_card(atom: &mut Atom, package_id: &str, ui: &mut egui::Ui) -> bool {
    let accent = ui.visuals().selection.stroke.color;
    let border = ui.visuals().widgets.noninteractive.bg_stroke.color;
    let raised = ui.visuals().window_fill;
    let weak = ui.visuals().weak_text_color();

    let mut delete = false;
    egui::Frame::new()
        .fill(raised)
        .stroke(egui::Stroke::new(1.0_f32, border))
        .corner_radius(METRICS.rounding)
        .inner_margin(METRICS.padding_small as i8)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_top(|ui| {
                badge(ui, kind_label(atom.kind), accent);
                ui.add_space(METRICS.padding_small);

                let content_w =
                    (ui.available_width() - METRICS.compact_size - METRICS.padding_small)
                        .max(5.0 * METRICS.padding);
                ui.vertical(|ui| {
                    ui.set_width(content_w);
                    match (atom.kind, atom.resource()) {
                        (AtomKind::Text, _) => {
                            ui.add(
                                egui::TextEdit::multiline(&mut atom.body)
                                    .frame(false)
                                    .desired_rows(1)
                                    .desired_width(content_w)
                                    .hint_text("Текст фрагмента"),
                            );
                        },
                        (AtomKind::Image, Some(id)) => {
                            ui.add(
                                egui::Image::new(format!("package://{}/{}", package_id, id.path()))
                                    .corner_radius(METRICS.rounding as f32)
                                    .max_width(content_w)
                                    .max_height(200.0),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(id.path())
                                        .size(METRICS.font_small)
                                        .color(weak),
                                )
                                .truncate()
                                .selectable(false),
                            );
                        },
                        (_, resource) => {
                            let name = resource
                                .map(|id| id.path().to_string())
                                .filter(|s| !s.is_empty())
                                .unwrap_or_else(|| "нет файла".to_string());
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(name).size(METRICS.font_body).color(weak),
                                )
                                .truncate()
                                .selectable(false),
                            );
                        },
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    delete = icon_button(
                        ui,
                        egui::Vec2::splat(METRICS.compact_size),
                        icon!(X),
                        weak,
                        ui.visuals().error_fg_color,
                        egui::Color32::TRANSPARENT,
                    )
                    .clicked();
                });
            });
        });
    delete
}

fn answers_section(question: &mut Question, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = METRICS.padding_small;
        let (r, w) = (question.right.len(), question.wrong.len());
        let weak = ui.visuals().weak_text_color();
        section_head(ui, "Ответы", weak, &format!("{r} верных · {w} неверных"));

        ui.columns(2, |cols| {
            answer_column(&mut cols[0], question, false);
            answer_column(&mut cols[1], question, true);
        });
    });
}

fn answer_column(ui: &mut egui::Ui, question: &mut Question, wrong: bool) {
    let accent = ui.visuals().selection.stroke.color;
    let danger = ui.visuals().error_fg_color;
    let base = ui.visuals().panel_fill;
    let bar = if wrong { mix(danger, base, 0.45) } else { accent };
    let head_color = if wrong { danger } else { accent };
    let (title, add_label) = if wrong {
        ("Неправильные", "+ Неправильный")
    } else {
        ("Правильные", "+ Правильный")
    };

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = METRICS.gap;
        let count = if wrong { question.wrong.len() } else { question.right.len() };
        section_head(ui, title, head_color, &count.to_string());

        let mut remove = None;
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = METRICS.gap_small;
            let list = if wrong { &mut question.wrong } else { &mut question.right };
            for (i, answer) in list.iter_mut().enumerate() {
                if answer_row(ui, answer, bar) {
                    remove = Some(i);
                }
            }
        });
        if let Some(i) = remove {
            if wrong {
                question.wrong.remove(i);
            } else {
                question.right.remove(i);
            }
        }

        if outlined_button(
            ui,
            egui::vec2(ui.available_width(), METRICS.button_height),
            add_label,
            head_color,
        )
        .clicked()
        {
            if wrong {
                let next = question.wrong.len() + 1;
                question.wrong.push(format!("Неправильный ответ #{next}"));
            } else {
                let next = question.right.len() + 1;
                question.right.push(format!("Правильный ответ #{next}"));
            }
        }
    });
}

fn answer_row(ui: &mut egui::Ui, answer: &mut String, bar_color: egui::Color32) -> bool {
    let border = ui.visuals().widgets.noninteractive.bg_stroke.color;
    let raised = ui.visuals().window_fill;

    let mut delete = false;
    let inner = egui::Frame::new()
        .fill(raised)
        .stroke(egui::Stroke::new(1.0_f32, border))
        .corner_radius(METRICS.rounding)
        .inner_margin(egui::Margin::symmetric(METRICS.padding_small as i8, METRICS.gap as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let text_w = (ui.available_width() - METRICS.compact_size - METRICS.padding_small)
                    .max(2.0 * METRICS.card_padding);
                ui.add_sized(
                    [text_w, 22.0],
                    egui::TextEdit::singleline(answer).frame(false).desired_width(text_w),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    delete = icon_button(
                        ui,
                        egui::Vec2::splat(METRICS.compact_size),
                        icon!(X),
                        ui.visuals().weak_text_color(),
                        ui.visuals().error_fg_color,
                        egui::Color32::TRANSPARENT,
                    )
                    .clicked();
                });
            });
        });

    // Coloured left bar.
    let rect = inner.response.rect;
    let bar =
        egui::Rect::from_min_size(rect.min, egui::vec2(METRICS.accent_bar_width, rect.height()));
    ui.painter().rect_filled(bar, egui::CornerRadius { nw: 4, sw: 4, ne: 0, se: 0 }, bar_color);
    delete
}

pub fn question_properties(ctx: &mut QuestionContext, ui: &mut egui::Ui) {
    let theme_name = {
        let theme_idx = ctx.idx().parent();
        ctx.package().get_theme(theme_idx).map(|theme| theme.name.clone()).unwrap_or_default()
    };

    let question = ctx.question();
    let fragments = question.scenario.len();
    let answers = question.right.len() + question.wrong.len();

    field_block(ui, |ui| {
        field_label(ui, "Стоимость");
        let mut value = question.price.to_string();
        if compact_input(ui, &mut value, "0").changed() {
            if let Ok(parsed) = value.parse::<usize>() {
                question.price = parsed;
            }
        }
    });

    field_block(ui, |ui| {
        field_label(ui, "Тип вопроса");
        question_type_selector(question, ui);
    });

    let info = question.info.get_or_insert_with(Default::default);
    text_field(ui, "Комментарий ведущего", &mut info.comments, "Не указан");
    text_field(ui, "Расширения", &mut info.extension, "");
    chips_field(ui, "Авторы", "+ Добавить автора", "question-authors", &mut info.authors);
    chips_field(ui, "Источники", "+ Добавить источник", "question-sources", &mut info.sources);

    ui.add_space(METRICS.gap_small);
    ui.separator();
    ui.add_space(METRICS.padding);
    section_label(ui, "Сводка");
    ui.add_space(METRICS.padding_small);
    meta_row(ui, "Фрагменты", &fragments.to_string());
    meta_row(ui, "Ответы", &answers.to_string());
    meta_row(ui, "Тема", &theme_name);
}

fn question_type_selector(question: &mut Question, ui: &mut egui::Ui) {
    const OPTIONS: [(&str, &str); 3] =
        [("Обычный вопрос", ""), ("Кот в мешке", "cat"), ("Аукцион", "auction")];
    let current = question.question_type.name.clone();

    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = METRICS.gap_tiny;
        for (label, name) in OPTIONS {
            let selected = current == name || (name.is_empty() && current == "simple");
            if type_option(ui, label, selected).clicked() {
                question.question_type.name = name.to_string();
            }
        }
    });
}

fn badge(ui: &mut egui::Ui, text: &str, accent: egui::Color32) {
    let base = ui.visuals().panel_fill;
    let border = mix(accent, base, 0.45);
    let font = egui::FontId::proportional(METRICS.font_small);
    let galley = ui.fonts(|f| f.layout_no_wrap(text.to_uppercase(), font, accent));
    let size = galley.size() + egui::vec2(METRICS.padding, METRICS.gap);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        egui::Color32::TRANSPARENT,
        egui::Stroke::new(1.0_f32, border),
        egui::StrokeKind::Inside,
    );
    ui.painter().galley(rect.center() - galley.size() / 2.0, galley, accent);
}

fn type_option(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let accent = ui.visuals().selection.stroke.color;
    let border = ui.visuals().widgets.noninteractive.bg_stroke.color;
    let border_strong = ui.visuals().widgets.hovered.bg_stroke.color;
    let text = ui.visuals().text_color();
    let weak = ui.visuals().weak_text_color();
    let (rect, resp) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), METRICS.button_height),
        egui::Sense::click(),
    );
    let hovered = resp.hovered();
    let (edge, fg, dot) = if selected {
        (accent, accent, accent)
    } else if hovered {
        (border_strong, text, weak)
    } else {
        (border, weak, border_strong)
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(METRICS.rounding),
        egui::Color32::TRANSPARENT,
        egui::Stroke::new(1.0_f32, edge),
        egui::StrokeKind::Inside,
    );
    let cy = rect.center().y;
    ui.painter().circle_filled(
        egui::pos2(rect.left() + METRICS.padding_small, cy),
        METRICS.gap_tiny,
        dot,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 2.0 * METRICS.padding_small, cy),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(METRICS.font_label),
        fg,
    );
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn kind_label(kind: AtomKind) -> &'static str {
    match kind {
        AtomKind::Text => "текст",
        AtomKind::Image => "изображение",
        AtomKind::Voice => "аудио",
        AtomKind::Video => "видео",
    }
}
