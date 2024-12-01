use std::fmt::Display;

use opensi_core::prelude::*;

use super::PropertyTable;

/// A generic error label.
pub fn error_label(error: impl Display, ui: &mut egui::Ui) {
    let text =
        egui::RichText::new(error.to_string()).color(ui.style().visuals.error_fg_color).size(24.0);
    ui.add(egui::Label::new(text).selectable(true).wrap());
}

/// A stub todo label.
pub fn todo_label(ui: &mut egui::Ui) {
    let text = egui::RichText::new("TODO")
        .background_color(ui.style().visuals.warn_fg_color)
        .strong()
        .size(24.0);
    ui.add(egui::Label::new(text).selectable(false).extend());
}

pub fn danger_button(text: impl Into<egui::WidgetText>, ui: &mut egui::Ui) -> egui::Response {
    ui.scope(|ui| {
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::DARK_RED;
        ui.style_mut().visuals.widgets.active.weak_bg_fill = egui::Color32::DARK_RED;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::RED;
        ui.style_mut().visuals.widgets.inactive.fg_stroke.color = egui::Color32::RED;
        ui.style_mut().visuals.widgets.active.fg_stroke.color = egui::Color32::RED;
        ui.style_mut().visuals.widgets.hovered.fg_stroke.color = egui::Color32::LIGHT_RED;
        ui.add(egui::Button::new(text))
    })
    .inner
}

pub fn unselectable_heading(text: impl Into<String>, ui: &mut egui::Ui) -> egui::Response {
    let text = egui::RichText::new(text).heading();
    unselectable_label(text, ui)
}

pub fn unselectable_label(text: impl Into<egui::WidgetText>, ui: &mut egui::Ui) -> egui::Response {
    ui.add(egui::Label::new(text).selectable(false))
}

pub fn string_list(
    id: impl Into<egui::Id>,
    list: &mut Vec<String>,
    ui: &mut egui::Ui,
) -> egui::Response {
    ui.push_id(id.into(), |ui| {
        ui.with_layout(
            egui::Layout::top_down(egui::Align::Min)
                .with_cross_justify(true)
                .with_main_align(egui::Align::Center),
            |ui| {
                if list.is_empty() {
                    unselectable_label("Пусто...", ui);
                } else {
                    ui.horizontal(|ui| {
                        let mut deleted_index = None;

                        for (index, item) in list.iter().enumerate() {
                            egui::Frame::none()
                                .rounding(4.0)
                                .inner_margin(egui::Margin { left: 4.0, ..Default::default() })
                                .fill(ui.style().visuals.widgets.inactive.bg_fill)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(item);
                                        if ui.small_button("❌").clicked() {
                                            deleted_index = Some(index);
                                        }
                                    });
                                });
                        }

                        if let Some(index) = deleted_index {
                            list.remove(index);
                        }
                    });
                }

                ui.spacing();

                let new_item_id = ui.id().with("new");
                let mut text = ui.memory_mut(|memory| {
                    memory.data.get_temp_mut_or_default::<String>(new_item_id).clone()
                });
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::exact(22.0))
                    .size(egui_extras::Size::remainder())
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            if ui.button("➕").clicked() && !text.is_empty() {
                                list.push(text.clone());
                                ui.memory_mut(|memory| {
                                    memory.data.remove_temp::<String>(new_item_id)
                                });
                            }
                        });

                        strip.cell(|ui| {
                            if ui.text_edit_singleline(&mut text).changed() {
                                ui.memory_mut(|memory| memory.data.insert_temp(new_item_id, text));
                            }
                        });
                    });
            },
        );
    })
    .response
}

pub fn info_edit(info: &mut Option<Info>, ui: &mut egui::Ui) {
    let Some(info) = info.as_mut() else {
        if ui.button("➕ Добавить информацию").clicked() {
            *info = Some(Default::default());
        }
        return;
    };

    PropertyTable::new("info-properties").show(ui, |mut properties| {
        properties.multiline_row("Авторы", 2, |ui| {
            string_list("info-properties-authors", &mut info.authors, ui)
        });
        properties.multiline_row("Источники", 2, |ui| {
            string_list("info-properties-sources", &mut info.sources, ui)
        });
        properties.row("Комментарий", |ui| ui.text_edit_singleline(&mut info.comments));
        properties.row("Расширения", |ui| ui.text_edit_singleline(&mut info.extension));
    });
}
