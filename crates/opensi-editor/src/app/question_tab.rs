use opensi_core::prelude::*;

use crate::{
    element::{PropertyTable, Sections, info_edit, unselectable_label},
    icon, icon_str, icon_string,
};

pub fn question_tab(question: &mut Question, ui: &mut egui::Ui) {
    Sections::new("question-sections")
        .line(egui_extras::Size::initial(200.0), 2)
        .line(egui_extras::Size::remainder(), 2)
        .show(ui, |mut body| {
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
                    question_scenario(question, ui);
                });
                line.section("Ответы", |ui| {
                    question_answers(question, ui);
                });
            });
        });
}

fn question_info_edit(question: &mut Question, ui: &mut egui::Ui) {
    PropertyTable::new("question-info-properties").show(ui, |mut properties| {
        properties.row(icon_str!(COINS, "Стоимость"), |ui| {
            ui.add(egui::DragValue::new(&mut question.price).range(0..=usize::MAX))
        });
        properties.row(icon_str!(STAR, "Тип вопроса"), |ui| {
            unselectable_label(format!("{:?}", question.question_type), ui)
        });
    });
}

fn question_scenario(question: &mut Question, ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
        for atom in &mut question.scenario {
            atom_ui(atom, ui);
        }
    });
}

fn question_answers(question: &mut Question, ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
        for answer in &mut question.right {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(icon_string!(CHECK_FAT, answer))
                        .color(ui.visuals().widgets.hovered.text_color()),
                )
                .selectable(false),
            );
        }
        for answer in &mut question.wrong {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(icon_string!(PLACEHOLDER, answer))
                        .color(ui.visuals().error_fg_color),
                )
                .selectable(false),
            );
        }
    });
}

fn atom_ui(atom: &mut Atom, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        match atom.kind {
            AtomKind::Image => unselectable_label(icon!(IMAGE), ui),
            AtomKind::Voice => unselectable_label(icon!(HEADPHONES), ui),
            AtomKind::Video => unselectable_label(icon!(VIDEO), ui),
            AtomKind::Text => unselectable_label(icon!(CHAT_CIRCLE_TEXT), ui),
        };

        match atom.kind {
            AtomKind::Text => {
                if atom.body.is_empty() {
                    ui.add(
                        egui::Label::new(egui::WidgetText::from("Пусто...").weak())
                            .selectable(false),
                    );
                } else {
                    ui.strong(&atom.body);
                }
            },
            _ => {
                unselectable_label(format!("{atom:?}"), ui);
            },
        }
    });
}
