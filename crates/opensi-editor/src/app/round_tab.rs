use opensi_core::prelude::*;

use crate::element::{
    card::{CardStyle, CardTable},
    error_label, info_edit, unselectable_heading, PropertyTable,
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
    PropertyTable::new("round-properties").show(ui, |mut properties| {
        properties.row("Название", |ui| ui.text_edit_singleline(&mut round.name));
        properties.row("Тип", |ui| {
            ui.add_enabled_ui(false, |ui| ui.label(format!("{:?}?", round.kind))).inner
        });
    });
}

fn round_themes(
    package: &mut Package,
    idx: RoundIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    let count = {
        let Some(round) = package.get_round(idx) else {
            return;
        };
        let max_theme_len =
            round.themes.iter().map(|theme| theme.questions.len()).max().unwrap_or_default();
        (max_theme_len + 2, round.themes.len() + 1)
    };

    CardTable::new("round-themes").show(ui, count, |mut row| {
        let idx = idx.theme(row.index());

        if package.contains_theme(idx) {
            if row.theme(package, idx, CardStyle::Important).clicked() {
                *selected = Some(idx.into());
            }

            for question_idx in 0..package.count_questions(idx).min(count.0 - 2) {
                let idx = idx.question(question_idx);
                if row.question(package, idx, CardStyle::Normal).clicked() {
                    *selected = Some(idx.into());
                }
            }

            if row.custom("➕ Новый вопрос", CardStyle::Weak).clicked() {
                package.allocate_question(idx);
            }
        } else {
            if row.custom("➕ Новая тема", CardStyle::Weak).clicked() {
                package.allocate_theme(idx.parent());
            }
        }
    });
}
