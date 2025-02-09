#![allow(dead_code)]

use std::fmt::Display;

use super::{PropertyTable, property::Properties};
use opensi_core::prelude::*;

#[macro_export]
macro_rules! icon {
    ($icon:ident) => {
        egui_phosphor::fill::$icon
    };
}

#[macro_export]
macro_rules! icon_str {
    ($icon:ident, $str:literal) => {
        const_format::formatcp!("{} {}", crate::icon!($icon), $str)
    };
}

#[macro_export]
macro_rules! icon_string {
    ($icon:ident, $string:expr) => {
        format!("{} {}", crate::icon!($icon), $string)
    };
}

#[macro_export]
macro_rules! icon_format {
    ($icon:ident, $fmt:literal $(,)? $($t:tt)*) => {
        format!("{} {}", crate::icon!($icon), format_args!($fmt, $($t,)*))
    };
}

/// A generic error label.
pub fn error_label(error: impl Display, ui: &mut egui::Ui) {
    let text = egui::RichText::new(icon_string!(WARNING, error))
        .color(ui.style().visuals.error_fg_color)
        .size(24.0);
    ui.add(egui::Label::new(text).selectable(true).wrap());
}

/// A stub todo label.
pub fn todo_label(ui: &mut egui::Ui) {
    let warn_bg = ui.style().visuals.warn_fg_color.linear_multiply(0.1);
    let text = egui::RichText::new("< TODO >")
        .monospace()
        .color(ui.style().visuals.warn_fg_color)
        .background_color(warn_bg)
        .size(24.0);
    ui.add(egui::Label::new(text).selectable(false).extend());
}

pub fn danger_button(text: impl Into<egui::WidgetText>, ui: &mut egui::Ui) -> egui::Response {
    ui.scope(|ui| {
        let error = ui.style().visuals.error_fg_color;
        let error_bg = error.linear_multiply(0.1);

        ui.style_mut().visuals.widgets.inactive.fg_stroke.color = error;
        ui.style_mut().visuals.widgets.active.fg_stroke.color = error;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = error_bg;
        ui.style_mut().visuals.widgets.hovered.bg_fill = error_bg;
        ui.style_mut().visuals.widgets.hovered.fg_stroke.color = error;
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
                            egui::Frame::new()
                                .corner_radius(4)
                                .inner_margin(egui::Margin { left: 4, ..Default::default() })
                                .fill(ui.style().visuals.widgets.inactive.bg_fill)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(item);
                                        if ui
                                            .add(
                                                egui::Button::new(icon!(X_CIRCLE))
                                                    .small()
                                                    .frame(false),
                                            )
                                            .clicked()
                                        {
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
                            if ui.button(icon!(PLUS_CIRCLE)).clicked() && !text.is_empty() {
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
        if ui.button(icon_str!(LIST_PLUS, "Добавить информацию")).clicked() {
            *info = Some(Default::default());
        }
        return;
    };

    PropertyTable::new("info-properties").show(ui, |mut properties| {
        info_properties(info, &mut properties);
    });
}

pub fn info_properties(info: &mut Info, properties: &mut Properties) {
    properties.multiline_row(icon_str!(USERS, "Авторы"), 2, |ui| {
        string_list("info-properties-authors", &mut info.authors, ui)
    });
    properties.multiline_row(icon_str!(ARCHIVE, "Источники"), 2, |ui| {
        string_list("info-properties-sources", &mut info.sources, ui)
    });
    properties.row(icon_str!(CHAT_DOTS, "Комментарий"), |ui| {
        ui.text_edit_singleline(&mut info.comments)
    });
    properties.row(icon_str!(PUZZLE_PIECE, "Расширения"), |ui| {
        ui.text_edit_singleline(&mut info.extension)
    });
}
