use egui_phosphor::fill as icon;
use opensi_core::prelude::*;
use std::borrow::Cow;

const UNKNOWN_ROUND: &'static str = "<Неизвестный раунд>";
const UNKNOWN_THEME: &'static str = "<Неизвестная тема>";
const UNKNOWN_QUESTION: &'static str = "<Неизвестный вопрос>";

/// Utility method to get a pretty name for a [`PackageNode`].
pub fn node_name<'a>(node: PackageNode, package: &'a Package) -> Cow<'a, str> {
    match node {
        PackageNode::Round(idx) => {
            package.get_round(idx).map(round_name).map(Cow::Owned).unwrap_or(UNKNOWN_ROUND.into())
        },
        PackageNode::Theme(idx) => {
            package.get_theme(idx).map(theme_name).map(Cow::Owned).unwrap_or(UNKNOWN_THEME.into())
        },
        PackageNode::Question(idx) => package
            .get_question(idx)
            .map(question_name)
            .map(Cow::Owned)
            .unwrap_or(UNKNOWN_QUESTION.into()),
    }
}

pub fn round_name(round: &Round) -> String {
    format!("{} {}", icon::ROWS, round.name)
}

pub fn theme_name(theme: &Theme) -> String {
    format!("{} {}", icon::STACK, theme.name)
}

pub fn question_name(question: &Question) -> String {
    format!("{} ({})", icon::NOTE, question.price)
}
