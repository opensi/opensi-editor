use opensi_core::prelude::*;

use crate::utils::todo_label;

pub fn theme_tab(_theme: &mut Theme, ui: &mut egui::Ui) {
    todo_label(ui);
}
