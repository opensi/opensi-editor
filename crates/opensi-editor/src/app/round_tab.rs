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

/// Workarea tab to edit round info and its themes.
pub fn round_tab(ctx: &mut PackageContext, idx: RoundIdx, ui: &mut egui::Ui) {
    let count = {
        let Some(round) = ctx.package().get_round(idx) else {
            return;
        };
        if round.themes.is_empty() {
            (1, 1)
        } else {
            let max_theme_len =
                round.themes.iter().map(|theme| theme.questions.len()).max().unwrap_or_default();
            (max_theme_len + 2, round.themes.len() + 1)
        }
    };

    CardTable::new("round-themes").show(ui, count, |mut row| {
        let idx = idx.theme(row.index());

        if ctx.package().contains_theme(idx) {
            if row.theme(ctx.package(), idx, CardStyle::Important).clicked() {
                ctx.select(idx.into());
            }

            for question_idx in 0..ctx.package().count_questions(idx).min(count.0 - 2) {
                let idx = idx.question(question_idx);
                if row.question(ctx.package(), idx, CardStyle::Normal).clicked() {
                    ctx.select(idx.into());
                }
            }

            if row.custom(icon_str!(FILE_PLUS, "Добавить вопрос"), CardStyle::Weak).clicked()
            {
                ctx.package().allocate_question(idx);
            }
        } else {
            if row.custom(icon_str!(STACK_PLUS, "Добавить тему"), CardStyle::Weak).clicked()
            {
                ctx.package().allocate_theme(idx.parent());
            }
        }
    });
}

pub fn round_properties(ctx: &mut PackageContext, idx: RoundIdx, ui: &mut egui::Ui) {
    let Some(round) = ctx.package().get_round_mut(idx) else {
        return;
    };

    Sections::new("round-properties")
        .line(egui_extras::Size::relative(0.75), 1)
        .line(egui_extras::Size::remainder(), 1)
        .show(ui, |mut body| {
            body.line(|mut line| {
                line.section("Раунд", |ui| {
                    round_edit(round, ui);
                });
            });
            body.line(|mut line| {
                line.section("Дополнительная информация", |ui| {
                    info_edit(&mut round.info, ui);
                });
            });
        });
}

fn round_edit(round: &mut Round, ui: &mut egui::Ui) {
    PropertyTable::new("round-properties").show(ui, |mut properties| {
        properties
            .row(icon!(STICKER), "Название", |ui| ui.text_edit_singleline(&mut round.name));
        properties.row(icon!(STAR), "Тип", |ui| {
            ui.add_enabled_ui(false, |ui| ui.label(format!("{:?}?", round.kind))).inner
        });
    });
}
