use egui::collapsing_header::CollapsingState;
use opensi_core::prelude::*;

use crate::{
    app::context::PackageContext,
    element::{node_context::PackageNodeContextMenu, node_name},
};

/// Ui for a whole [`Package`] in a form of a tree.
///
/// It can add new rounds, themes and questions, edit
/// names/prices of existing ones and select them.
pub fn package_tree(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    ui.vertical_centered_justified(|ui| {
        let text = egui::RichText::new(&ctx.package().name).heading();
        if ui.add(egui::Label::new(text).sense(egui::Sense::click()).selectable(false)).clicked() {
            ctx.deselect();
        }
    });

    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        tree_node_ui(ctx, None, ui);
    });
}

/// Recursive [`PackageNode`] ui.
fn tree_node_ui<'a>(ctx: &mut PackageContext, node: Option<PackageNode>, ui: &mut egui::Ui) {
    fn node_button(
        ctx: &mut PackageContext,
        node: PackageNode,
        is_selected: bool,
        ui: &mut egui::Ui,
    ) -> bool {
        let node_name = node_name(node, ctx.package());
        let button =
            egui::Button::new(node_name.as_ref()).frame(false).fill(egui::Color32::TRANSPARENT);
        let response = ui.add(button);
        let response = if is_selected { response.highlight() } else { response };

        PackageNodeContextMenu { package: ctx.package(), node }.show(&response, ui);

        return response.clicked();
    }

    let Some(node) = node else {
        ui.push_id(format!("package-tree"), |ui| {
            if ctx.package().rounds.is_empty() {
                ui.weak("Нет раундов");
            } else {
                for index in 0..ctx.package().rounds.len() {
                    tree_node_ui(ctx, Some(index.into()), ui);
                }
            }
        });
        ui.allocate_response(ui.available_size(), egui::Sense::click()).context_menu(|ui| {
            if ui.button(format!("➕ Добавить раунд")).clicked() {
                ctx.package().allocate_round();
                ui.close_menu();
            }
        });
        return;
    };

    let id = egui::Id::new(node.index()).with(ui.id());
    let is_selected = ctx.selected().is_some_and(|selected| selected == node);
    match node {
        PackageNode::Round(idx) => {
            let mut state = CollapsingState::load_with_default_open(ui.ctx(), id, true)
                .show_header(ui, |ui| {
                    if node_button(ctx, node, is_selected, ui) {
                        ctx.select(node);
                    };
                });

            if !state.is_open() && ctx.selected().is_some_and(|selected| {
                matches!(
                    selected,
                    PackageNode::Question(QuestionIdx { round_index, .. })
                    | PackageNode::Theme(ThemeIdx { round_index, .. })
                    | PackageNode::Round(RoundIdx { index: round_index, .. }) if round_index == *idx
                )
            }) {
                state.set_open(true);
            }

            state.body(|ui| {
                for theme_index in 0..ctx
                    .package()
                    .get_round(idx)
                    .map(|round| round.themes.len())
                    .unwrap_or_default()
                {
                    tree_node_ui(ctx, Some(idx.theme(theme_index).into()), ui);
                }
            });
        },
        PackageNode::Theme(idx) => {
            let mut state = CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    if node_button(ctx, node, is_selected, ui) {
                        ctx.select(node);
                    };
                });

            if !state.is_open() && ctx.selected().is_some_and(|selected| {
                matches!(
                    selected,
                    PackageNode::Question(QuestionIdx { theme_index, .. })
                    | PackageNode::Theme(ThemeIdx { index: theme_index, .. }) if theme_index == *idx
                )
            }) {
                state.set_open(true);
            }

            state.body(|ui| {
                for question_index in 0..ctx
                    .package()
                    .get_theme(idx)
                    .map(|theme| theme.questions.len())
                    .unwrap_or_default()
                {
                    tree_node_ui(ctx, Some(idx.question(question_index).into()), ui);
                }
            });
        },
        PackageNode::Question(idx) => {
            if node_button(ctx, idx.into(), is_selected, ui) {
                ctx.select(node);
            }
        },
    }
}
