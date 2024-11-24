use opensi_core::prelude::*;

use crate::{
    card::CardKind,
    utils::{error_label, info_edit, simple_row, unselectable_heading, unselectable_label},
};

/// Workarea tab to edit round info and its themes.
pub fn round_tab(
    package: &mut Package,
    idx: RoundIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    ui.vertical(|ui| {
        ui.allocate_ui(egui::vec2(ui.available_width(), 200.0), |ui| {
            egui_extras::StripBuilder::new(ui)
                .sizes(egui_extras::Size::remainder().at_most(500.0), 2)
                .cell_layout(egui::Layout::top_down(egui::Align::Min))
                .horizontal(|mut strip| {
                    let Some(round) = package.get_round_mut(idx) else {
                        let error = format!("Невозможно найти раунд с индексом {idx}");
                        strip.cell(|ui| {
                            error_label(error, ui);
                        });
                        strip.empty();
                        return;
                    };

                    strip.cell(|ui| {
                        unselectable_heading("Раунд", ui);
                        ui.separator();
                        round_edit(round, ui);
                    });

                    strip.cell(|ui| {
                        unselectable_heading("Дополнительная информация", ui);
                        ui.separator();
                        info_edit(&mut round.info, ui);
                    });
                });
        });

        unselectable_heading("Темы", ui);
        ui.separator();
        round_themes(package, idx, selected, ui);
    });
}

fn round_edit(round: &mut Round, ui: &mut egui::Ui) {
    egui_extras::TableBuilder::new(ui)
        .id_salt("round-edit")
        .column(egui_extras::Column::auto())
        .column(egui_extras::Column::remainder())
        .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
        .striped(false)
        .body(|mut body| {
            simple_row("Название", 20.0, &mut body, |ui| {
                ui.text_edit_singleline(&mut round.name);
            });
            simple_row("Вариант", 20.0, &mut body, |ui| {
                // TODO: variant enum
                unselectable_label(format!("TODO: {:?}", round.variant), ui);
            });
        });
}

fn round_themes(
    package: &mut Package,
    idx: RoundIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    egui_extras::StripBuilder::new(ui)
        .size(egui_extras::Size::remainder())
        .size(egui_extras::Size::exact(30.0))
        .cell_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                let Some(round) = package.get_round_mut(idx) else {
                    return;
                };

                egui::ScrollArea::both().show(ui, |ui| {
                    let max_theme_len = round
                        .themes
                        .iter()
                        .map(|theme| theme.questions.len())
                        .max()
                        .unwrap_or_default();

                    egui_extras::TableBuilder::new(ui)
                        .id_salt("rounds")
                        .vscroll(false)
                        .columns(egui_extras::Column::remainder(), max_theme_len + 2)
                        .cell_layout(egui::Layout::centered_and_justified(
                            egui::Direction::LeftToRight,
                        ))
                        .body(|body| {
                            body.rows(100.0, round.themes.len(), |mut row| {
                                let theme_idx = idx.theme(row.index());
                                let Some(theme) = round.themes.get_mut(*theme_idx) else {
                                    return;
                                };

                                row.col(|ui| {
                                    if CardKind::Theme(theme).show(ui).clicked() {
                                        *selected = Some(theme_idx.into());
                                    }
                                });

                                for (question_index, question) in theme.questions.iter().enumerate()
                                {
                                    row.col(|ui| {
                                        if CardKind::Question(question).show(ui).clicked() {
                                            *selected =
                                                Some(theme_idx.question(question_index).into());
                                        }
                                    });
                                }

                                row.col(|ui| {
                                    if CardKind::New.show(ui).clicked() {
                                        theme.questions.push(Question {
                                            price: theme.guess_next_question_price(),
                                            ..Default::default()
                                        });
                                    }
                                });
                            });
                        });
                });
            });

            strip.cell(|ui| {
                if ui.button("➕ Добавить новую тему").clicked() {
                    package.allocate_theme(idx);
                }
            });
        });
}
