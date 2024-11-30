use opensi_core::prelude::*;

use crate::element::todo_label;

pub fn question_tab(_question: &mut Question, ui: &mut egui::Ui) {
    todo_label(ui);
}
