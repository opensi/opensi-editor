use opensi_core::Question;

use crate::utils::todo_label;

pub fn question_tab(_question: &mut Question, ui: &mut egui::Ui) {
    todo_label(ui);
}
