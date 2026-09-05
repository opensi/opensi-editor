use crate::{
    element::{ModalExt, props::section_label},
    icon_str,
};

pub fn about_modal(ui: &mut egui::Ui) {
    ui.modal_title_desc(icon_str!(GRADUATION_CAP, "OpenSI Editor"), "редактор пакетов вопросов");

    ui.modal_body(|ui| {
        let weak = ui.visuals().weak_text_color();
        section_label(ui, "Авторы");
        ui.add_space(6.0);
        for author in env!("CARGO_PKG_AUTHORS").split(':') {
            let author = author.trim();
            let (name, mail) = match author.split_once('<') {
                Some((name, mail)) => (name.trim(), mail.trim_end_matches('>').trim()),
                None => (author, ""),
            };
            ui.horizontal(|ui| {
                ui.label(name);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Label::new(egui::RichText::new(mail).color(weak)));
                });
            });
        }
        ui.add_space(16.0);

        section_label(ui, "Репозиторий");
        ui.add_space(6.0);
        let url = env!("CARGO_PKG_REPOSITORY");
        ui.hyperlink_to(url.trim_start_matches("https://github.com/"), url);
        ui.add_space(16.0);

        section_label(ui, "Версия");
        ui.add_space(6.0);
        ui.label(concat!("v", env!("CARGO_PKG_VERSION")));
    });

    ui.modal_buttons(|ui| {
        ui.modal_confirm(icon_str!(CHECK, "Закрыть"));
    });
}
