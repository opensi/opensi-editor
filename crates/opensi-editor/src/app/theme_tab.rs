use opensi_core::prelude::*;

use crate::element::{
    card::{CardStyle, CardTable},
    info_edit, PropertyTable, Sections,
};

pub fn theme_tab(
    package: &mut Package,
    idx: ThemeIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    Sections::new("theme-sections")
        .line(egui_extras::Size::relative(0.2).at_least(200.0), 2)
        .line(egui_extras::Size::relative(0.8), 1)
        .show(ui, |mut body| {
            body.line(|mut line| {
                let Some(theme) = package.get_theme_mut(idx) else {
                    return;
                };
                line.section("Тема", |ui| {
                    theme_edit(theme, ui);
                });
                line.section("Дополнительная информация", |ui| {
                    info_edit(&mut theme.info, ui);
                });
            });
            body.line(|mut line| {
                line.section("Вопросы", |ui| {
                    theme_questions(package, idx, selected, ui);
                });
            });
        });
}

fn theme_edit(theme: &mut Theme, ui: &mut egui::Ui) {
    PropertyTable::new("theme-properties").show(ui, |mut properties| {
        properties.row("Название", |ui| ui.text_edit_singleline(&mut theme.name));
    });
}

fn theme_questions(
    package: &mut Package,
    idx: ThemeIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    let Some(theme) = package.get_theme_mut(idx) else {
        return;
    };

    CardTable::new("theme-questions").show(ui, (1, theme.questions.len() + 1), |mut row| {
        let idx = idx.question(row.index());
        if package.contains_question(idx) {
            if row.question(package, idx, CardStyle::Important).clicked() {
                *selected = Some(idx.into());
            }
        } else {
            if row.custom("➕ Новый вопрос", CardStyle::Weak).clicked() {
                package.allocate_question(idx.parent());
            }
        }
    });
}
