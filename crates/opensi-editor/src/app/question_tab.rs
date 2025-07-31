use opensi_core::prelude::*;

use crate::{
    app::context::QuestionContext,
    element::{PropertyTable, Sections, danger_button, info_edit, unselectable_label},
    icon, icon_str,
};

pub fn question_tab(ctx: &mut QuestionContext, ui: &mut egui::Ui) {
    Sections::new("question-sections").line(egui_extras::Size::remainder(), 2).show(
        ui,
        |mut body| {
            body.line(|mut line| {
                line.section("Сценарий", |ui| {
                    question_scenario(ctx, ui);
                });
                line.section("Ответы", |ui| {
                    question_answers(ctx.question(), ui);
                });
            });
        },
    );
}

pub fn question_properties(ctx: &mut QuestionContext, ui: &mut egui::Ui) {
    Sections::new("question-properties")
        .line(egui_extras::Size::relative(0.75), 1)
        .line(egui_extras::Size::remainder(), 1)
        .show(ui, |mut body| {
            body.line(|mut line| {
                line.section("Вопрос", |ui| {
                    question_info_edit(ctx.question(), ui);
                });
            });
            body.line(|mut line| {
                line.section("Дополнительная информация", |ui| {
                    info_edit(&mut ctx.question().info, ui);
                });
            });
        });
}

fn question_info_edit(question: &mut Question, ui: &mut egui::Ui) {
    PropertyTable::new("question-info-properties").show(ui, |mut properties| {
        properties.row(icon!(COINS), "Стоимость", |ui| {
            ui.add(egui::DragValue::new(&mut question.price).range(0..=usize::MAX))
        });
        properties.row(icon!(STAR), "Тип вопроса", |ui| {
            unselectable_label(question.question_type.to_string(), ui)
        });
    });
}

fn question_scenario(ctx: &mut QuestionContext, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.scope(|ui| {
            ui.style_mut().spacing.item_spacing.y = 10.0;
            let id = ctx.package().id.clone();
            for atom in &mut ctx.question().scenario {
                atom_ui(atom, &id, ui);
            }
        });

        ui.add_space(20.0);

        egui_extras::TableBuilder::new(ui)
            .cell_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown)
                    .with_main_justify(false),
            )
            .columns(egui_extras::Column::remainder(), 2)
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        if ui.button(icon_str!(IMAGE, "Добавить изображение")).clicked()
                        {
                            let idx = ctx.idx();
                            ctx.pick_new_image_for(idx);
                        }
                    });
                    row.col(|ui| {
                        if ui.button(icon_str!(CHAT_CIRCLE_TEXT, "Добавить текст")).clicked()
                        {
                            ctx.question()
                                .scenario
                                .push(Atom { kind: AtomKind::Text, ..Atom::default() });
                        }
                    });
                });
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        if ui.button(icon_str!(HEADPHONES, "Добавить аудио")).clicked()
                        {
                            ctx.question()
                                .scenario
                                .push(Atom { kind: AtomKind::Voice, ..Atom::default() });
                        }
                    });
                    row.col(|ui| {
                        if ui.button(icon_str!(VIDEO, "Добавить видео")).clicked() {
                            ctx.question()
                                .scenario
                                .push(Atom { kind: AtomKind::Video, ..Atom::default() });
                        }
                    });
                });
            });
    });
}

fn atom_ui(atom: &mut Atom, package_id: &str, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let icon = match atom.kind {
            AtomKind::Image => icon!(IMAGE),
            AtomKind::Voice => icon!(HEADPHONES),
            AtomKind::Video => icon!(VIDEO),
            AtomKind::Text => icon!(CHAT_CIRCLE_TEXT),
        };
        ui.add(
            egui::Label::new(
                egui::RichText::new(icon).size(20.0).color(ui.visuals().hyperlink_color),
            )
            .selectable(false),
        );
        let start_position = ui.next_widget_position() + egui::vec2(-18.0, 11.0);

        match (atom.kind, atom.resource()) {
            (AtomKind::Text, _) => {
                ui.add(
                    egui::TextEdit::multiline(&mut atom.body)
                        .desired_rows(2)
                        .desired_width(ui.available_width())
                        .margin(egui::Margin::symmetric(10, 6)),
                );
            },
            (AtomKind::Image, Some(id)) => {
                ui.add(
                    egui::Image::new(format!("package://{}/{}", package_id, id.path()))
                        .corner_radius(8.0)
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width()),
                );
            },
            _ => {
                unselectable_label(format!("{atom:?}"), ui);
            },
        }

        ui.painter().vline(
            start_position.x,
            start_position.y..=(start_position.y + ui.min_size().y - 26.0),
            ui.visuals().noninteractive().fg_stroke,
        );
    });
}

fn question_answers(question: &mut Question, ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
        for n in 0..question.right.len() {
            answer_ui(question, n, false, ui);
        }
        for n in 0..question.wrong.len() {
            answer_ui(question, n, true, ui);
        }

        ui.add_space(20.0);

        egui_extras::StripBuilder::new(ui)
            .cell_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown)
                    .with_main_justify(false),
            )
            .sizes(egui_extras::Size::remainder(), 2)
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    if ui.button(icon_str!(CHECK, "Добавить правильный")).clicked()
                    {
                        question
                            .right
                            .push(format!("Правильный ответ #{}", question.right.len() + 1));
                    }
                });

                strip.cell(|ui| {
                    if ui.button(icon_str!(X, "Добавить неправильный")).clicked()
                    {
                        question
                            .wrong
                            .push(format!("Неправильный ответ #{}", question.wrong.len() + 1));
                    }
                });
            });
    });
}

fn answer_ui(question: &mut Question, n: usize, is_wrong: bool, ui: &mut egui::Ui) {
    if (is_wrong && n >= question.wrong.len()) || (!is_wrong && n >= question.right.len()) {
        return;
    }

    ui.push_id(ui.id().with("answer").with(is_wrong).with(n), |ui| {
        let edit_id = ui.id().with("edit");
        let is_edit = ui.memory(|memory| memory.data.get_temp::<bool>(edit_id)).unwrap_or_default();

        let (icon, color) = if is_wrong {
            (icon!(X), ui.visuals().error_fg_color)
        } else {
            (icon!(CHECK), ui.visuals().hyperlink_color)
        };

        ui.set_max_height(30.0);
        ui.set_max_width(ui.available_width());
        egui_extras::StripBuilder::new(ui)
            .clip(true)
            .size(egui_extras::Size::exact(20.0))
            .size(egui_extras::Size::remainder())
            .size(egui_extras::Size::exact(50.0))
            .cell_layout(
                egui::Layout::left_to_right(egui::Align::Min).with_main_align(egui::Align::Min),
            )
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    ui.add(
                        egui::Label::new(egui::WidgetText::from(icon).color(color))
                            .selectable(false),
                    );
                });

                strip.cell(|ui| {
                    let answer =
                        if is_wrong { &mut question.wrong[n] } else { &mut question.right[n] };

                    if is_edit {
                        let response = ui.add(
                            egui::TextEdit::singleline(answer)
                                .desired_width(ui.available_width() - n as f32 * 2.0),
                        );
                        response.request_focus();

                        if response.has_focus()
                            && ui.input(|input| input.key_pressed(egui::Key::Enter))
                        {
                            ui.memory_mut(|memory| memory.data.insert_temp(edit_id, false));
                        }
                    } else {
                        ui.add(egui::Label::new(egui::RichText::new(answer.as_str()).color(color)));
                    }
                });

                strip.cell(|ui| {
                    if is_edit {
                        if ui.button(icon!(FLOPPY_DISK)).clicked() {
                            ui.memory_mut(|memory| memory.data.insert_temp(edit_id, false));
                        }
                    } else {
                        if ui.button(icon!(PENCIL)).clicked() {
                            ui.memory_mut(|memory| memory.data.insert_temp(edit_id, true));
                        }
                    }

                    if danger_button(icon!(TRASH), ui).clicked() {
                        if is_wrong {
                            question.wrong.remove(n);
                        } else {
                            question.right.remove(n);
                        }
                    }
                });
            });
    });
}
