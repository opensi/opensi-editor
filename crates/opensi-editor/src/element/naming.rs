use opensi_core::prelude::*;
use std::borrow::Cow;

use crate::{icon_format, icon_string};

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
    icon_string!(SQUARES_FOUR, round.name)
}

pub fn theme_name(theme: &Theme) -> String {
    icon_string!(SQUARE, theme.name)
}

pub fn question_name(question: &Question) -> String {
    icon_format!(QUESTION, "({})", question.price)
}

/// Whether a question has authored content (answers or scenario).
pub fn question_is_filled(question: &Question) -> bool {
    !question.right.is_empty() || !question.scenario.is_empty()
}

/// TODO: fluent.
/// Russian plural selector: `one` (1), `few` (2-4), `many` (else).
pub fn plural(n: usize, one: &'static str, few: &'static str, many: &'static str) -> &'static str {
    let (m10, m100) = (n % 10, n % 100);
    if m10 == 1 && m100 != 11 {
        one
    } else if (2..=4).contains(&m10) && !(12..=14).contains(&m100) {
        few
    } else {
        many
    }
}

/// Min and max over question prices in a single pass (`None` when empty).
pub fn price_range(prices: impl Iterator<Item = usize>) -> Option<(usize, usize)> {
    prices.fold(None, |acc, p| match acc {
        None => Some((p, p)),
        Some((lo, hi)) => Some((lo.min(p), hi.max(p))),
    })
}

/// "min–max" across question prices (or "-" when empty).
pub fn price_range_label(prices: impl Iterator<Item = usize>) -> String {
    match price_range(prices) {
        Some((lo, hi)) => format!("{lo}–{hi}"),
        None => "—".to_string(),
    }
}
