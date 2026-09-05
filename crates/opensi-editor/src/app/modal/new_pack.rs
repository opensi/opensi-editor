use crate::{element::ModalExt, icon_str};

pub fn new_pack_modal(ui: &mut egui::Ui) -> bool {
    let mut confirmed = false;
    ui.modal_title(icon_str!(PENCIL_SIMPLE_LINE, "Перезаписать текущий пак ?"));
    ui.modal_buttons(|ui| {
        ui.modal_danger(icon_str!(PROHIBIT, "Отмена"));
        if ui.modal_confirm(icon_str!(CHECK, "Перезаписать")).clicked() {
            confirmed = true;
        }
    });
    confirmed
}
