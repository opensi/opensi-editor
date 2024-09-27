use std::borrow::Cow;

use egui::collapsing_header::CollapsingState;
use opensi_core::{Package, PackageNode};

/// Ui for a whole [`Package`] in a form of a tree.
///
/// It can add new rounds, themes and questions, edit
/// names/prices of existing ones and select them.
pub fn package_tree(package: &mut Package, ui: &mut egui::Ui) {
    let name = package.name.as_ref().map(|name| name.as_str()).unwrap_or("Новый пакет вопросов");

    ui.vertical_centered_justified(|ui| {
        ui.heading(name);
    });

    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        tree_node_ui(package, None, ui);
    });
}

/// Recursive [`PackageNode`] ui.
fn tree_node_ui<'a>(package: &mut Package, node: Option<PackageNode>, ui: &mut egui::Ui) {
    fn plus_button(ui: &mut egui::Ui) -> bool {
        ui.vertical_centered_justified(|ui| ui.button("➕").clicked()).inner
    }

    fn node_button(package: &mut Package, node: PackageNode, ui: &mut egui::Ui) -> bool {
        #[derive(Default)]
        struct Result {
            new_name: Option<String>,
            is_selected: bool,
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
            let response = ui.button(node_name.as_ref());

            response.context_menu(|ui| {
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
                if ui.button("❌ Удалить").clicked() {
                    result.is_deleted = true;
                    ui.close_menu();
                }
            });
            if response.clicked() {
                result.is_selected = true;
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
                    tree_node_ui(package, Some(PackageNode::Round { index }), ui);
                }
            }
        });
        if plus_button(ui) {
            package.allocate_round();
        }
        return;
    };

    let id = egui::Id::new(node.index()).with(ui.id());
    match node {
        node @ PackageNode::Round { index } => {
            CollapsingState::load_with_default_open(ui.ctx(), id, true)
                .show_header(ui, |ui| {
                    if node_button(package, node, ui) {
                        // TODO: selected round
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
                            ui,
                        );
                    }
                    if plus_button(ui) {
                        package.allocate_theme(index);
                    }
                });
        },
        node @ PackageNode::Theme { round_index, index } => {
            CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    if node_button(package, node, ui) {
                        // TODO: selected theme
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
                            ui,
                        );
                    }
                    if plus_button(ui) {
                        package.allocate_question(round_index, index);
                    }
                });
        },
        PackageNode::Question { round_index, theme_index, index } => {
            if node_button(package, PackageNode::Question { round_index, theme_index, index }, ui) {
                // TODO: selected question
            }
        },
    }
}

/// Utility method to get a button name for a [`PackageNode`].
fn node_name<'a>(node: PackageNode, package: &'a Package) -> Cow<'a, str> {
    match node {
        PackageNode::Round { index } => package
            .get_round(index)
            .map(|round| round.name.as_str())
            .unwrap_or("<Неизвестный раунд>")
            .into(),
        PackageNode::Theme { round_index, index } => package
            .get_theme(round_index, index)
            .map(|theme| theme.name.as_str())
            .unwrap_or("<Неизвестная тема>")
            .into(),
        PackageNode::Question { round_index, theme_index, index } => package
            .get_question(round_index, theme_index, index)
            .map(|question| format!("🗛 ({})", question.price).into())
            .unwrap_or("<Неизвестный вопрос>".into()),
    }
}
