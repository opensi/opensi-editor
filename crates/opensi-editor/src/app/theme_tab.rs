use opensi_core::prelude::*;

use crate::element::{
    card::{CardStyle, CardTable},
    error_label, info_edit, unselectable_heading, PropertyTable,
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
        let index = row.index();
        let Some(question) = package.get_question(idx.question(index)) else {
            if row.custom("➕ Новый вопрос", CardStyle::Weak).clicked() {
                package.allocate_question(idx);
            }
            return;
        };

        if row.question(question, CardStyle::Normal).clicked() {
            *selected = Some(idx.question(index).into());
        }
    });
}
