use egui::collapsing_header::CollapsingState;
use opensi_core::{Package, PackageNode};

use crate::utils::node_name;

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
            PackageNode::Round { index } => format!("tree-node-round-{index}"),
            PackageNode::Theme { round_index, index } => {
                format!("tree-node-theme-{round_index}-{index}")
            },
            PackageNode::Question { round_index, theme_index, index } => {
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
            match node {
                PackageNode::Round { index } => {
                    package.allocate_theme(index);
                },
                PackageNode::Theme { round_index, index } => {
                    package.allocate_question(round_index, index);
                },
                PackageNode::Question { .. } => {},
            }
        }
        if result.is_duplicated {
            match node {
                PackageNode::Round { index } => {
                    package.duplicate_round(index);
                },
                PackageNode::Theme { round_index, index } => {
                    package.duplicate_theme(round_index, index);
                },
                PackageNode::Question { round_index, theme_index, index } => {
                    package.duplicate_question(round_index, theme_index, index);
                },
            }
        }
        if result.is_deleted {
            match node {
                PackageNode::Round { index } => {
                    package.remove_round(index);
                },
                PackageNode::Theme { round_index, index } => {
                    package.remove_theme(round_index, index);
                },
                PackageNode::Question { round_index, theme_index, index } => {
                    package.remove_question(round_index, theme_index, index);
                },
            }
        }
        if let Some(new_name) = result.new_name {
            match node {
                PackageNode::Round { index } => {
                    if let Some(round) = package.get_round_mut(index) {
                        round.name = new_name;
                    };
                },
                PackageNode::Theme { round_index, index } => {
                    if let Some(theme) = package.get_theme_mut(round_index, index) {
                        theme.name = new_name;
                    };
                },
                PackageNode::Question { round_index, theme_index, index } => {
                    if let Some(question) =
                        package.get_question_mut(round_index, theme_index, index)
                    {
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
            if package.rounds.rounds.is_empty() {
                ui.weak("Нет раундов");
            } else {
                for index in 0..package.rounds.rounds.len() {
                    tree_node_ui(package, Some(PackageNode::Round { index }), selected, ui);
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
        PackageNode::Round { index } => {
            CollapsingState::load_with_default_open(ui.ctx(), id, true)
                .show_header(ui, |ui| {
                    if node_button(package, node, is_selected, ui) {
                        *selected = Some(node);
                    };
                })
                .body(|ui| {
                    for theme_index in 0..package
                        .get_round(index)
                        .map(|round| round.themes.themes.len())
                        .unwrap_or_default()
                    {
                        tree_node_ui(
                            package,
                            Some(PackageNode::Theme { round_index: index, index: theme_index }),
                            selected,
                            ui,
                        );
                    }
                });
        },
        PackageNode::Theme { round_index, index } => {
            CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    if node_button(package, node, is_selected, ui) {
                        *selected = Some(node);
                    };
                })
                .body(|ui| {
                    for question_index in 0..package
                        .get_theme(round_index, index)
                        .map(|theme| theme.questions.questions.len())
                        .unwrap_or_default()
                    {
                        tree_node_ui(
                            package,
                            Some(PackageNode::Question {
                                round_index,
                                theme_index: index,
                                index: question_index,
                            }),
                            selected,
                            ui,
                        );
                    }
                });
        },
        PackageNode::Question { round_index, theme_index, index } => {
            if node_button(
                package,
                PackageNode::Question { round_index, theme_index, index },
                is_selected,
                ui,
            ) {
                *selected = Some(node);
            }
        },
    }
}
