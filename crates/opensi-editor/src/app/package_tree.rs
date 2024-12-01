use egui::collapsing_header::CollapsingState;
use opensi_core::prelude::*;

use crate::element::{node_context::PackageNodeContextMenu, node_name};

/// Ui for a whole [`Package`] in a form of a tree.
///
/// It can add new rounds, themes and questions, edit
/// names/prices of existing ones and select them.
pub fn package_tree(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    ui.vertical_centered_justified(|ui| {
        let text = egui::RichText::new(&package.name).heading();
        if ui.add(egui::Label::new(text).sense(egui::Sense::click()).selectable(false)).clicked() {
            *selected = None;
        }
    });

    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        tree_node_ui(package, None, selected, ui);
    });
}

/// Recursive [`PackageNode`] ui.
fn tree_node_ui<'a>(
    package: &mut Package,
    node: Option<PackageNode>,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    fn node_button(
        package: &mut Package,
        node: PackageNode,
        is_selected: bool,
        ui: &mut egui::Ui,
    ) -> bool {
        let node_name = node_name(node, package);
        let button = egui::Button::new(node_name.as_ref())
            .selected(is_selected)
            .fill(egui::Color32::TRANSPARENT);
        let response = ui.add(button);

        PackageNodeContextMenu { package, node }.show(&response, ui);

        return response.clicked();
    }

    let Some(node) = node else {
        ui.push_id(format!("package-tree"), |ui| {
            if package.rounds.is_empty() {
                ui.weak("Нет раундов");
            } else {
                for index in 0..package.rounds.len() {
                    tree_node_ui(package, Some(index.into()), selected, ui);
                }
            }
        });
        ui.allocate_response(ui.available_size(), egui::Sense::click()).context_menu(|ui| {
            if ui.button(format!("➕ Добавить раунд")).clicked() {
                package.allocate_round();
                ui.close_menu();
            }
        });
        return;
    };

    let id = egui::Id::new(node.index()).with(ui.id());
    let is_selected = selected.is_some_and(|selected| selected == node);
    match node {
        PackageNode::Round(idx) => {
            CollapsingState::load_with_default_open(ui.ctx(), id, true)
                .show_header(ui, |ui| {
                    if node_button(package, node, is_selected, ui) {
                        *selected = Some(node);
                    };
                })
                .body(|ui| {
                    for theme_index in 0..package
                        .get_round(idx)
                        .map(|round| round.themes.len())
                        .unwrap_or_default()
                    {
                        tree_node_ui(package, Some(idx.theme(theme_index).into()), selected, ui);
                    }
                });
        },
        PackageNode::Theme(idx) => {
            CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    if node_button(package, node, is_selected, ui) {
                        *selected = Some(node);
                    };
                })
                .body(|ui| {
                    for question_index in 0..package
                        .get_theme(idx)
                        .map(|theme| theme.questions.len())
                        .unwrap_or_default()
                    {
                        tree_node_ui(
                            package,
                            Some(idx.question(question_index).into()),
                            selected,
                            ui,
                        );
                    }
                });
        },
        PackageNode::Question(idx) => {
            if node_button(package, idx.into(), is_selected, ui) {
                *selected = Some(node);
            }
        },
    }
}
