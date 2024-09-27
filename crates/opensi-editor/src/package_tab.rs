use opensi_core::Package;

/// Workarea tab to edit package info.
pub fn package_tab(package: &mut Package, ui: &mut egui::Ui) {
    egui_extras::TableBuilder::new(ui)
        .column(egui_extras::Column::auto())
        .column(egui_extras::Column::remainder().at_least(400.0))
        .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
        .striped(false)
        .body(|mut body| {
            package_edit_row("Название", &mut body, |ui| {
                ui.text_edit_singleline(&mut package.name);
            });
            package_edit_row("Сложность", &mut body, |ui| {
                ui.add(egui::DragValue::new(&mut package.difficulty).range(0..=10));
            });
            package_edit_row("Ограничения", &mut body, |ui| {
                ui.text_edit_singleline(&mut package.restriciton);
            });
            package_edit_row("Дата создания", &mut body, |ui| {
                ui.text_edit_singleline(&mut package.date);
            });
            package_edit_row("Издатель", &mut body, |ui| {
                ui.text_edit_singleline(&mut package.publisher);
            });
            package_edit_row("Язык", &mut body, |ui| {
                ui.text_edit_singleline(&mut package.language);
            });
        });

    ui.collapsing("Метаданные", |ui| {
        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::remainder().at_least(400.0))
            .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
            .striped(true)
            .body(|mut body| {
                package_edit_row("ID пакета", &mut body, |ui| {
                    ui.label(&package.id);
                });
                package_edit_row("Версия", &mut body, |ui| {
                    ui.label(format!("{:.1}", package.version));
                });
            });
    });
}

fn package_edit_row(
    label: impl AsRef<str>,
    body: &mut egui_extras::TableBody,
    content: impl FnOnce(&mut egui::Ui),
) {
    body.row(20.0, |mut row| {
        row.col(|ui| {
            ui.label(label.as_ref());
        });
        row.col(content);
    });
}
