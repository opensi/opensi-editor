use std::{borrow::Cow, fmt::Display};

use opensi_core::prelude::*;

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

/// Utility method to get a button name for a [`PackageNode`].
pub fn node_name<'a>(node: PackageNode, package: &'a Package) -> Cow<'a, str> {
    match node {
        PackageNode::Round(idx) => package
            .get_round(idx)
            .map(|round| round.name.as_str())
            .unwrap_or("<Неизвестный раунд>")
            .into(),
        PackageNode::Theme(idx) => package
            .get_theme(idx)
            .map(|theme| theme.name.as_str())
            .unwrap_or("<Неизвестная тема>")
            .into(),
        PackageNode::Question(idx) => package
            .get_question(idx)
            .map(|question| format!("🗛 ({})", question.price).into())
            .unwrap_or("<Неизвестный вопрос>".into()),
    }
}

pub fn unselectable_heading(text: impl Into<String>, ui: &mut egui::Ui) -> egui::Response {
    let text = egui::RichText::new(text).heading();
    unselectable_label(text, ui)
}

pub fn unselectable_label(text: impl Into<egui::WidgetText>, ui: &mut egui::Ui) -> egui::Response {
    ui.add(egui::Label::new(text).selectable(false))
}

pub fn string_list(id: impl Into<egui::Id>, list: &mut Vec<String>, ui: &mut egui::Ui) {
    ui.push_id(id.into(), |ui| {
        ui.vertical(|ui| {
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

            ui.horizontal(|ui| {
                let new_item_id = ui.id().with("new");
                let mut text = ui.memory_mut(|memory| {
                    memory.data.get_temp_mut_or_default::<String>(new_item_id).clone()
                });

                if ui.button("➕").clicked() && !text.is_empty() {
                    list.push(text.clone());
                    ui.memory_mut(|memory| memory.data.remove_temp::<String>(new_item_id));
                }

                if ui.text_edit_singleline(&mut text).changed() {
                    ui.memory_mut(|memory| memory.data.insert_temp(new_item_id, text));
                }
            });
        });
    });
}
