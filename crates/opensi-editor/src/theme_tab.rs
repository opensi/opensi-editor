use opensi_core::prelude::*;

use crate::{
    card::CardKind,
    utils::{error_label, info_edit, simple_row, unselectable_heading},
};

pub fn theme_tab(
    package: &mut Package,
    idx: ThemeIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    ui.vertical(|ui| {
        ui.allocate_ui(egui::vec2(ui.available_width(), 200.0), |ui| {
            egui_extras::StripBuilder::new(ui)
                .sizes(egui_extras::Size::remainder().at_most(500.0), 2)
                .cell_layout(egui::Layout::top_down(egui::Align::Min))
                .horizontal(|mut strip| {
                    let Some(theme) = package.get_theme_mut(idx) else {
                        let error = format!("Невозможно найти тему с индексом {idx}");
                        strip.cell(|ui| {
                            error_label(error, ui);
                        });
                        strip.empty();
                        return;
                    };

                    strip.cell(|ui| {
                        unselectable_heading("Тема", ui);
                        ui.separator();
                        theme_edit(theme, ui);
                    });

                    strip.cell(|ui| {
                        unselectable_heading("Дополнительная информация", ui);
                        ui.separator();
                        info_edit(&mut theme.info, ui);
                    });
                });
        });

        unselectable_heading("Вопросы", ui);
        ui.separator();
        theme_questions(package, idx, selected, ui);
    });
}

fn theme_edit(theme: &mut Theme, ui: &mut egui::Ui) {
    egui_extras::TableBuilder::new(ui)
        .id_salt("theme-edit")
        .column(egui_extras::Column::auto())
        .column(egui_extras::Column::remainder())
        .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
        .striped(false)
        .body(|mut body| {
            simple_row("Название", 20.0, &mut body, |ui| {
                ui.text_edit_singleline(&mut theme.name);
            });
        });
}

fn theme_questions(
    package: &mut Package,
    idx: ThemeIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    egui::ScrollArea::horizontal().show(ui, |ui| {
        let Some(theme) = package.get_theme_mut(idx) else {
            return;
        };

        ui.set_max_height(100.0);

        egui_extras::StripBuilder::new(ui)
            .sizes(egui_extras::Size::remainder().at_least(200.0), theme.questions.len() + 1)
            .cell_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight))
            .horizontal(|mut strip| {
                for (question_index, question) in theme.questions.iter().enumerate() {
                    strip.cell(|ui| {
                        if CardKind::Question(question).show(ui).clicked() {
                            *selected = Some(idx.question(question_index).into());
                        }
                    });
                }

                strip.cell(|ui| {
                    if CardKind::New.show(ui).clicked() {
                        theme.questions.push(Question {
                            price: theme.guess_next_question_price(),
                            ..Default::default()
                        });
                    }
                });
            });
    });
}
