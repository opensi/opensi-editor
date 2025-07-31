use opensi_core::prelude::*;

use crate::{
    app::context::PackageContext,
    element::{
        PropertyTable, Sections,
        card::{CardStyle, CardTable},
        info_edit,
    },
    icon, icon_str,
};

pub fn theme_tab(ctx: &mut PackageContext, idx: ThemeIdx, ui: &mut egui::Ui) {
    let Some(theme) = ctx.package().get_theme_mut(idx) else {
        return;
    };

    CardTable::new("theme-questions").show(ui, (1, theme.questions.len() + 1), |mut row| {
        let idx = idx.question(row.index());
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

pub fn theme_properties(ctx: &mut PackageContext, idx: ThemeIdx, ui: &mut egui::Ui) {
    let Some(theme) = ctx.package().get_theme_mut(idx) else {
        return;
    };

    Sections::new("theme-properties")
        .line(egui_extras::Size::relative(0.75), 1)
        .line(egui_extras::Size::remainder(), 1)
        .show(ui, |mut body| {
            body.line(|mut line| {
                line.section("Тема", |ui| {
                    theme_edit(theme, ui);
                });
            });
            body.line(|mut line| {
                line.section("Дополнительная информация", |ui| {
                    info_edit(&mut theme.info, ui);
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
