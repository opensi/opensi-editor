use std::{borrow::Cow, fmt::Display};

use opensi_core::{Package, PackageNode};

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
        PackageNode::Round { index } => package
            .get_round(index)
            .map(|round| round.name.as_str())
            .unwrap_or("<Неизвестный раунд>")
            .into(),
        PackageNode::Theme { round_index, index } => package
            .get_theme(round_index, index)
            .map(|theme| theme.name.as_str())
            .unwrap_or("<Неизвестная тема>")
            .into(),
        PackageNode::Question { round_index, theme_index, index } => package
            .get_question(round_index, theme_index, index)
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
