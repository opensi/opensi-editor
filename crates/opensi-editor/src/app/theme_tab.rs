use opensi_core::prelude::*;

use crate::{
    element::{
        PropertyTable, Sections,
        card::{CardStyle, CardTable},
        info_edit,
    },
    icon, icon_str,
};

pub fn theme_tab(
    package: &mut Package,
    idx: ThemeIdx,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    Sections::new("theme-sections")
        .line(egui_extras::Size::initial(200.0), 2)
        .line(egui_extras::Size::remainder(), 1)
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
        properties
            .row(icon!(STICKER), "Название", |ui| ui.text_edit_singleline(&mut theme.name));
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
            if row.custom(icon_str!(FILE_PLUS, "Добавить вопрос"), CardStyle::Weak).clicked()
            {
                package.allocate_question(idx.parent());
            }
        }
    });
}
