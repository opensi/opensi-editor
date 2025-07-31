use opensi_core::prelude::*;

use crate::{
    app::context::ThemeContext,
    element::{
        PropertyTable, Sections,
        card::{CardStyle, CardTable},
        info_edit,
    },
    icon, icon_str,
};

pub fn theme_tab(ctx: &mut ThemeContext, ui: &mut egui::Ui) {
    CardTable::new("theme-questions").show(ui, (1, ctx.theme().questions.len() + 1), |mut row| {
        let idx = ctx.idx().question(row.index());
        if ctx.package().contains_question(idx) {
            if row.question(ctx.package(), idx, CardStyle::Important).clicked() {
                ctx.select(idx.into());
            }
        } else {
            if row.custom(icon_str!(FILE_PLUS, "Добавить вопрос"), CardStyle::Weak).clicked()
            {
                ctx.package().allocate_question(idx.parent());
            }
        }
    });
}

pub fn theme_properties(ctx: &mut ThemeContext, ui: &mut egui::Ui) {
    Sections::new("theme-properties")
        .line(egui_extras::Size::relative(0.75), 1)
        .line(egui_extras::Size::remainder(), 1)
        .show(ui, |mut body| {
            body.line(|mut line| {
                line.section("Тема", |ui| {
                    theme_edit(ctx.theme(), ui);
                });
            });
            body.line(|mut line| {
                line.section("Дополнительная информация", |ui| {
                    info_edit(&mut ctx.theme().info, ui);
                });
            });
        });
}

fn theme_edit(theme: &mut Theme, ui: &mut egui::Ui) {
    PropertyTable::new("theme-properties").show(ui, |mut properties| {
        properties
            .row(icon!(STICKER), "Название", |ui| ui.text_edit_singleline(&mut theme.name));
    });
}
