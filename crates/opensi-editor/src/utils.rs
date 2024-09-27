use std::{borrow::Cow, fmt::Display};

use opensi_core::{Package, PackageNode};

/// A generic error label.
pub fn error_label(error: impl Display, ui: &mut egui::Ui) {
    let text = egui::RichText::new(error.to_string()).color(egui::Color32::RED).size(24.0);
    ui.add(egui::Label::new(text).selectable(true).wrap());
}

/// A stub todo label.
pub fn todo_label(ui: &mut egui::Ui) {
    let text =
        egui::RichText::new("TODO").background_color(egui::Color32::YELLOW).strong().size(24.0);
    ui.add(egui::Label::new(text).selectable(false).extend());
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
