use itertools::Itertools;
use opensi_core::prelude::*;

use crate::element::{
    danger_button, string_list, unselectable_heading, unselectable_label, PropertyTable,
};

/// Workarea tab to edit package info.
pub fn package_tab(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.allocate_ui(egui::vec2(ui.available_width(), 200.0), |ui| {
            egui_extras::StripBuilder::new(ui)
                .sizes(egui_extras::Size::remainder().at_most(500.0), 2)
                .cell_layout(egui::Layout::top_down_justified(egui::Align::Min))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        unselectable_heading("Информация о пакете", ui);
                        ui.separator();
                        package_info_edit(package, ui);
                    });
                    strip.cell(|ui| {
                        unselectable_heading("Метаданные", ui);
                        ui.separator();
                        package_metadata_edit(package, ui);
                    });
                });
        });

        ui.allocate_ui(egui::vec2(1020.0, ui.available_height()), |ui| {
            unselectable_heading("Раунды", ui);
            ui.separator();
            ui.push_id("package_rounds", |ui| {
                package_rounds(package, selected, ui);
            });
        });
    });
}

fn package_info_edit(package: &mut Package, ui: &mut egui::Ui) {
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
            .multiline_row("Тэги", 2, |ui| string_list("package-tags", &mut package.tags, ui))
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
    fn round_card(round: &Round, ui: &mut egui::Ui) -> egui::Response {
        let mut frame = egui::Frame::default()
            .inner_margin(16.0)
            .outer_margin(egui::Margin::symmetric(0.0, 4.0))
            .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
            .rounding(8.0)
            .begin(ui);
        frame.content_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            unselectable_heading(&round.name, ui);
            ui.separator();
            let theme_names = if round.themes.is_empty() {
                "Пусто".to_string()
            } else {
                round.themes.iter().map(|theme| &theme.name).join(", ")
            };
            unselectable_label(egui::RichText::new(theme_names).italics(), ui);
        });
        let rect =
            frame.content_ui.min_rect() + frame.frame.inner_margin + frame.frame.outer_margin;
        let response = ui.allocate_rect(rect, egui::Sense::click());
        if response.hovered() {
            frame.frame.stroke = ui.style().visuals.widgets.active.bg_stroke;
            frame.frame.fill = ui.style().visuals.widgets.active.weak_bg_fill;
        }
        frame.paint(ui);
        response
    }

    egui_extras::StripBuilder::new(ui)
        .size(egui_extras::Size::remainder())
        .size(egui_extras::Size::exact(30.0))
        .cell_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                let button_size = 20.0;
                egui_extras::TableBuilder::new(ui)
                    .id_salt("rounds")
                    .column(egui_extras::Column::remainder())
                    .column(egui_extras::Column::exact(button_size))
                    .cell_layout(
                        egui::Layout::top_down_justified(egui::Align::Center)
                            .with_main_wrap(false)
                            .with_cross_justify(true)
                            .with_cross_align(egui::Align::Center),
                    )
                    .body(|mut body| {
                        for index in 0..package.rounds.len() {
                            body.row((button_size + 4.0) * 3.0, |mut row| {
                                row.col(|ui| {
                                    let Some(round) = package.get_round_mut(index) else {
                                        return;
                                    };
                                    if round_card(round, ui).clicked() {
                                        *selected = Some(index.into());
                                    }
                                });
                                row.col(|ui| {
                                    ui.add_space(4.0);
                                    if ui.button("✏").on_hover_text("Редактировать").clicked()
                                    {
                                        *selected = Some(index.into());
                                    }
                                    if ui.button("🗐").on_hover_text("Дублировать").clicked()
                                    {
                                        package.duplicate_round(index);
                                    }
                                    if danger_button("❌", ui).on_hover_text("Удалить").clicked()
                                    {
                                        package.remove_round(index);
                                    }
                                });
                            });
                        }
                    });
            });

            strip.cell(|ui| {
                if ui.button("➕ Добавить новый раунд").clicked() {
                    package.allocate_round();
                }
            });
        });
}
