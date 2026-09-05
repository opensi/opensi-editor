use opensi_core::prelude::*;

use crate::{
    element::{
        ModalExt, ModalWrapper,
        menu::{menu_divider, menu_item, menu_item_danger, style_menu},
    },
    icon, icon_str,
};

/// Context menu for [`PackageNode`].
pub struct PackageNodeContextMenu<'p> {
    pub package: &'p mut Package,
    pub node: PackageNode,
}

impl PackageNodeContextMenu<'_> {
    pub fn show(self, source: &egui::Response, ui: &mut egui::Ui) {
        let is_question = matches!(self.node, PackageNode::Question(..));
        let (change_glyph, change_label): (&str, &str) = if is_question {
            (icon!(COINS), "Изменить цену")
        } else {
            (icon!(PENCIL), "Переименовать")
        };
        let add: Option<(&str, &str)> = match self.node {
            PackageNode::Round(_) => Some((icon!(PLUS), "Добавить тему")),
            PackageNode::Theme(_) => Some((icon!(PLUS), "Добавить вопрос")),
            PackageNode::Question(_) => None,
        };
        // Accusative labels per node kind ("Дублировать тему", "Удалить раунд", ...).
        let (duplicate_label, delete_label) = match self.node {
            PackageNode::Round(_) => ("Дублировать раунд", "Удалить раунд"),
            PackageNode::Theme(_) => ("Дублировать тему", "Удалить тему"),
            PackageNode::Question(_) => ("Дублировать вопрос", "Удалить вопрос"),
        };
        let modal_title: &str = if is_question {
            icon_str!(COINS, "Изменить цену")
        } else {
            icon_str!(PENCIL, "Переименовать")
        };
        let new_value_id = source.id.with(egui::Id::new("new-value"));

        let mut modal = ModalWrapper::new(ui.ctx(), source.id.with("modal"));

        source.context_menu(|ui| {
            style_menu(ui);

            if let Some((glyph, label)) = add {
                if menu_item(ui, glyph, label, "").clicked() {
                    self.package.allocate_node(self.node.child(0).unwrap());
                    ui.close_menu();
                }
                menu_divider(ui);
            }

            if menu_item(ui, change_glyph, change_label, "").clicked() {
                let value = match self.node {
                    PackageNode::Round(idx) => self
                        .package
                        .get_round(idx)
                        .map(|round| round.name.clone())
                        .unwrap_or_default(),
                    PackageNode::Theme(idx) => self
                        .package
                        .get_theme(idx)
                        .map(|theme| theme.name.clone())
                        .unwrap_or_default(),
                    PackageNode::Question(idx) => self
                        .package
                        .get_question(idx)
                        .map(|question| question.price.to_string())
                        .unwrap_or_default(),
                };
                ui.memory_mut(|memory| memory.data.insert_temp(new_value_id, value));
                ui.close_menu();
                modal.open();
            }
            if menu_item(ui, icon!(COPY), duplicate_label, "").clicked() {
                self.package.duplicate_node(self.node);
                ui.close_menu();
            }
            menu_divider(ui);
            if menu_item_danger(ui, icon!(TRASH), delete_label, "").clicked() {
                self.package.remove_node(self.node);
                ui.close_menu();
            }
        });

        modal.show(ui.ctx(), |ui| {
            let mut is_renaming_done = false;

            ui.modal_title(modal_title);
            ui.modal_body(|ui| {
                let body = match self.node {
                    PackageNode::Round(_) => "Введите новое название для раунда:",
                    PackageNode::Theme(_) => "Введите новое название для темы:",
                    PackageNode::Question(_) => "Введите новую цену для вопроса:",
                };
                ui.label(body);

                let mut new_value = ui
                    .memory(|memory| memory.data.get_temp::<String>(new_value_id))
                    .unwrap_or_default();
                let response = ui.add(
                    egui::TextEdit::singleline(&mut new_value).id_salt(source.id.with("edit")),
                );
                response.request_focus();

                if response.changed() {
                    if is_question {
                        new_value.retain(|c| c.is_digit(10));
                    }
                    ui.memory_mut(|memory| memory.data.insert_temp(new_value_id, new_value));
                }

                is_renaming_done = ui.input(|input| input.key_pressed(egui::Key::Enter));
            });
            ui.modal_buttons(|ui| {
                if ui.modal_danger(icon_str!(PROHIBIT, "Отмена")).clicked() {
                    is_renaming_done = false;
                }
                if ui.modal_confirm(icon_str!(CHECK, "Подтвердить")).clicked() {
                    is_renaming_done = true;
                }
            });

            if is_renaming_done {
                ui.close_modal();
                let new_value = ui
                    .memory(|memory| memory.data.get_temp::<String>(new_value_id))
                    .unwrap_or_default();

                match self.node {
                    PackageNode::Round(idx) => {
                        if let Some(round) = self.package.get_round_mut(idx) {
                            round.name = new_value;
                        };
                    },
                    PackageNode::Theme(idx) => {
                        if let Some(theme) = self.package.get_theme_mut(idx) {
                            theme.name = new_value;
                        };
                    },
                    PackageNode::Question(idx) => {
                        if let Some(question) = self.package.get_question_mut(idx) {
                            if let Ok(new_price) = new_value.parse() {
                                question.price = new_price;
                            }
                        };
                    },
                }
            }
        });
    }
}
