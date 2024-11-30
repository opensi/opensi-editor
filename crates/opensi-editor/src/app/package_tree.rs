use egui::collapsing_header::CollapsingState;
use opensi_core::prelude::*;

use crate::element::node_name;

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
        #[derive(Default)]
        struct Result {
            new_name: Option<String>,
            is_selected: bool,
            is_duplicated: bool,
            is_populated: bool,
            is_deleted: bool,
        }
        let id = match node {
            PackageNode::Round(RoundIdx { index }) => format!("tree-node-round-{index}"),
            PackageNode::Theme(ThemeIdx { round_index, index }) => {
                format!("tree-node-theme-{round_index}-{index}")
            },
            PackageNode::Question(QuestionIdx { round_index, theme_index, index }) => {
                format!("tree-node-question-{round_index}-{theme_index}-{index}")
            },
        };
        let id = egui::Id::new(id);
        let mut result = Result::default();
        let is_question = matches!(node, PackageNode::Question { .. });

        if let Some(mut new_name) = ui.memory(|memory| memory.data.get_temp::<String>(id)) {
            // renaming in process
            let response = ui.text_edit_singleline(&mut new_name);

            let is_renaming_done = ui.input(|input| input.key_pressed(egui::Key::Enter));
            let is_exiting = is_renaming_done || !response.has_focus();

            if is_renaming_done {
                result.new_name = Some(new_name);
            } else if response.changed() {
                if is_question {
                    new_name.retain(|c| c.is_digit(10));
                }
                ui.memory_mut(|memory| memory.data.insert_temp(id, new_name));
            }

            if is_exiting {
                ui.memory_mut(|memory| memory.data.remove_temp::<String>(id));
                ui.ctx().request_repaint();
            }
        } else {
            // regular button
            let node_name = node_name(node, package);
            let button = egui::Button::new(node_name.as_ref())
                .selected(is_selected)
                .fill(egui::Color32::TRANSPARENT);
            let response = ui.add(button);

            response.context_menu(|ui| {
                if let Some(add_text) = match node {
                    PackageNode::Round { .. } => Some("Добавить тему"),
                    PackageNode::Theme { .. } => Some("Добавить вопрос"),
                    PackageNode::Question { .. } => None,
                } {
                    if ui.button(format!("➕ {add_text}")).clicked() {
                        result.is_populated = true;
                        ui.close_menu();
                    }
                    ui.separator();
                }

                let change_text = if is_question {
                    "Изменить цену"
                } else {
                    "Переименовать"
                };
                if ui.button(format!("✏ {}", change_text)).clicked() {
                    ui.memory_mut(|memory| {
                        let mut renaming = node_name.to_string();
                        if is_question {
                            renaming.retain(|c| c.is_digit(10));
                        }
                        memory.data.insert_temp(id, renaming);
                    });
                    response.request_focus();
                    ui.close_menu();
                }
                if ui.button("🗐 Дублировать").clicked() {
                    result.is_duplicated = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("❌ Удалить").clicked() {
                    result.is_deleted = true;
                    ui.close_menu();
                }
            });
            if response.clicked() {
                result.is_selected = true;
            }
        }

        if result.is_populated {
            if let Some(parent) = node.parent() {
                package.allocate_node(parent);
            }
        }
        if result.is_duplicated {
            package.duplicate_node(node);
        }
        if result.is_deleted {
            package.remove_node(node);
        }
        if let Some(new_name) = result.new_name {
            match node {
                PackageNode::Round(idx) => {
                    if let Some(round) = package.get_round_mut(idx) {
                        round.name = new_name;
                    };
                },
                PackageNode::Theme(idx) => {
                    if let Some(theme) = package.get_theme_mut(idx) {
                        theme.name = new_name;
                    };
                },
                PackageNode::Question(idx) => {
                    if let Some(question) = package.get_question_mut(idx) {
                        if let Ok(new_price) = new_name.parse() {
                            question.price = new_price;
                        }
                    };
                },
            }
        }

        return result.is_selected;
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
