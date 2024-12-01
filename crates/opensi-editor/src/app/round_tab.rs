use opensi_core::prelude::*;

use crate::element::{
    card::{CardStyle, CardTable},
    info_edit, PropertyTable, Sections,
};

/// Workarea tab to edit round info and its themes.
pub fn round_tab(
    package: &mut Package,
    idx: RoundIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    Sections::new("round-sections")
        .line(egui_extras::Size::relative(0.2).at_least(200.0), 2)
        .line(egui_extras::Size::relative(0.8), 1)
        .show(ui, |mut body| {
            body.line(|mut line| {
                let Some(round) = package.get_round_mut(idx) else {
                    return;
                };
                line.section("Раунд", |ui| {
                    round_edit(round, ui);
                });
                line.section("Дополнительная информация", |ui| {
                    info_edit(&mut round.info, ui);
                });
            });
            body.line(|mut line| {
                line.section("Темы", |ui| {
                    round_themes(package, idx, selected, ui);
                });
            });
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
        if round.themes.is_empty() {
            (1, 1)
        } else {
            let max_theme_len =
                round.themes.iter().map(|theme| theme.questions.len()).max().unwrap_or_default();
            (max_theme_len + 2, round.themes.len() + 1)
        }
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
