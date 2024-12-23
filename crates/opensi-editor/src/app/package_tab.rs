use opensi_core::prelude::*;

use crate::element::{
    card::{CardStyle, CardTable},
    info_properties, string_list, PropertyTable, Sections,
};

/// Workarea tab to edit package info.
pub fn package_tab(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    Sections::new("package-sections")
        .line(egui_extras::Size::initial(400.0), 2)
        .line(egui_extras::Size::remainder(), 1)
        .show(ui, |mut body| {
            body.line(|mut line| {
                line.section("Пакет вопросов", |ui| {
                    package_info_edit(package, ui);
                });
                line.section("Метаданные", |ui| {
                    package_metadata_edit(package, ui);
                });
            });
            body.line(|mut line| {
                line.section("Раунды", |ui| {
                    package_rounds(package, selected, ui);
                });
            });
        });
}

fn package_info_edit(package: &mut Package, ui: &mut egui::Ui) {
    let tags: &mut Vec<String> = &mut package.tags.iter().filter_map(|tag| tag.body.clone()).collect();
    PropertyTable::new("package-info-properties").show(ui, |mut properties| {
        properties.row("Название", |ui| ui.text_edit_singleline(&mut package.name));
        properties.row("Сложность", |ui| {
            ui.add(egui::DragValue::new(&mut package.difficulty).range(0..=10))
        });
        properties
            .row("Ограничения", |ui| ui.text_edit_singleline(&mut package.restriction));
        properties
            .row("Дата создания", |ui| ui.text_edit_singleline(&mut package.date));
        properties.row("Издатель", |ui| ui.text_edit_singleline(&mut package.publisher));
        properties.row("Язык", |ui| ui.text_edit_singleline(&mut package.language));
        properties
            .multiline_row("Тэги", 2, |ui| string_list("package-tags", tags, ui));

        info_properties(&mut package.info, &mut properties);
    });
}

fn package_metadata_edit(package: &Package, ui: &mut egui::Ui) {
    PropertyTable::new("package-metadata-properties").readonly(true).show(ui, |mut properties| {
        properties.row("ID пакета", |ui| ui.label(&package.id));
        properties
            .row("Версия пакета", |ui| ui.label(format!("{:.1}", package.version)));
    });
}

fn package_rounds(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    CardTable::new("package-rounds").show(ui, (1, package.rounds.len() + 1), |mut row| {
        let idx = row.index();
        if package.contains_round(idx) {
            if row.round(package, idx, CardStyle::Important).clicked() {
                *selected = Some(idx.into());
            }
        } else {
            if row.custom("➕ Новый раунд", CardStyle::Weak).clicked() {
                package.allocate_round();
            }
        }
    });
}
