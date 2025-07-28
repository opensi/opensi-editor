use opensi_core::prelude::*;

use crate::{
    element::{PropertyTable, Sections, danger_button, info_edit, unselectable_label},
    icon, icon_str,
};

pub fn question_tab(package: &mut Package, idx: QuestionIdx, ui: &mut egui::Ui) {
    let package_id = package.id.clone();

    Sections::new("question-sections")
        .line(egui_extras::Size::initial(200.0), 2)
        .line(egui_extras::Size::remainder(), 2)
        .show(ui, |mut body| {
            let Some(question) = package.get_question_mut(idx) else {
                return;
            };
            body.line(|mut line| {
                line.section("Вопрос", |ui| {
                    question_info_edit(question, ui);
                });
                line.section("Дополнительная информация", |ui| {
                    info_edit(&mut question.info, ui);
                });
            });
            body.line(|mut line| {
                line.section("Сценарий", |ui| {
                    question_scenario(question, &package_id, ui);
                });
                line.section("Ответы", |ui| {
                    question_answers(question, ui);
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
            unselectable_label(format!("{:?}", question.question_type), ui)
        });
    });
}

fn question_scenario(question: &mut Question, package_id: &str, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for atom in &mut question.scenario {
            atom_ui(atom, package_id, ui);
        }
    });
}

fn atom_ui(atom: &mut Atom, package_id: &str, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        match atom.kind {
            AtomKind::Image => unselectable_label(icon!(IMAGE), ui),
            AtomKind::Voice => unselectable_label(icon!(HEADPHONES), ui),
            AtomKind::Video => unselectable_label(icon!(VIDEO), ui),
            AtomKind::Text => unselectable_label(icon!(CHAT_CIRCLE_TEXT), ui),
        };

        match (atom.kind, atom.resource()) {
            (AtomKind::Text, _) => {
                if atom.body.is_empty() {
                    ui.add(
                        egui::Label::new(egui::WidgetText::from("Пусто...").weak())
                            .selectable(false),
                    );
                } else {
                    ui.strong(&atom.body);
                }
            },
            (AtomKind::Image, Some(id)) => {
                ui.add(
                    egui::Image::new(format!("package://{}/{}", package_id, id.path()))
                        .fit_to_original_size(1.0)
                        .max_width(ui.available_width()),
                );
            },
            _ => {
                unselectable_label(format!("{atom:?}"), ui);
            },
        }
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

        ui.horizontal_top(|ui| {
            if ui.button(icon_str!(CHECK_FAT, "Добавить правильный")).clicked() {
                question.right.push(format!("Правильный ответ #{}", question.right.len() + 1));
            }

            if ui.button(icon_str!(PLACEHOLDER, "Добавить неправильный")).clicked()
            {
                question.wrong.push(format!("Неправильный ответ #{}", question.wrong.len() + 1));
            }
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
            (icon!(PLACEHOLDER), ui.visuals().error_fg_color)
        } else {
            (icon!(CHECK_FAT), ui.visuals().widgets.hovered.text_color())
        };

        ui.set_max_height(22.0);

        egui_extras::StripBuilder::new(ui)
            .size(egui_extras::Size::initial(20.0))
            .size(egui_extras::Size::remainder())
            .size(egui_extras::Size::initial(50.0))
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
                        let response =
                            ui.add(egui::TextEdit::singleline(answer).desired_width(f32::INFINITY));
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
