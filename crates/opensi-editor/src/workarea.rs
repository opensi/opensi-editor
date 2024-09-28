use crate::{
    package_tab, question_tab, round_tab, theme_tab,
    utils::{error_label, node_name},
};
use opensi_core::{Package, PackageNode};

/// UI for general area of [`Package`] editing.
pub fn workarea(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        breadcrumbs(package, selected, ui);

        ui.add_space(16.0);

        ui.vertical_centered_justified(|ui| {
            selected_tab(package, selected, ui);
        });
    });
}

/// Tab ui based on what package node is selected.
fn selected_tab(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    match selected {
        &mut Some(PackageNode::Round { index }) => {
            if let Some(round) = package.get_round_mut(index) {
                round_tab::round_tab(round, ui);
            } else {
                let error = format!("Невозможно найти раунд с индексом {index}");
                error_label(error, ui);
            }
        },
        &mut Some(PackageNode::Theme { round_index, index }) => {
            if let Some(theme) = package.get_theme_mut(round_index, index) {
                theme_tab::theme_tab(theme, ui);
            } else {
                let error = format!(
                    "Невозможно найти тему с индексом {index} (раунд с индексом {round_index})"
                );
                error_label(error, ui);
            }
        },
        &mut Some(PackageNode::Question { round_index, theme_index, index }) => {
            if let Some(question) = package.get_question_mut(round_index, theme_index, index) {
                question_tab::question_tab(question, ui);
            } else {
                let error = [
                    format!("Невозможно найти вопрос с индексом {index}"),
                    format!("(раунд с индексом {round_index}, тема с индексом {theme_index})"),
                ]
                .join(" ");
                error_label(error, ui);
            }
        },
        None => {
            package_tab::package_tab(package, selected, ui);
        },
    }
}

/// Selection breadcrumbs ui.
fn breadcrumbs(package: &Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    fn breadcrumb(text: impl AsRef<str>, ui: &mut egui::Ui) -> bool {
        let text = egui::RichText::new(text.as_ref()).size(20.0);
        let response =
            ui.add(egui::Label::new(text).extend().sense(egui::Sense::click()).selectable(false));
        response.clicked()
    }

    fn breadcrump_separator(ui: &mut egui::Ui) {
        ui.add_space(8.0);
        let text = egui::RichText::new("/").size(8.0).weak();
        ui.add(egui::Label::new(text).wrap().selectable(false));
        ui.add_space(8.0);
    }

    fn root_breadcrumb(package: &Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
        if breadcrumb(format!("🏠 {}", package.name), ui) {
            *selected = None;
        }
    }

    fn node_breadcrumb(
        node: PackageNode,
        package: &Package,
        selected: &mut Option<PackageNode>,
        ui: &mut egui::Ui,
    ) {
        let name = node_name(node, package);
        if breadcrumb(name, ui) {
            *selected = Some(node);
        }
    }

    ui.horizontal(|ui| {
        root_breadcrumb(package, selected, ui);

        match *selected {
            Some(node @ PackageNode::Round { .. }) => {
                breadcrump_separator(ui);
                node_breadcrumb(node, package, selected, ui);
            },
            Some(node @ PackageNode::Theme { .. }) => {
                breadcrump_separator(ui);
                node_breadcrumb(node.get_parent().unwrap(), package, selected, ui);
                breadcrump_separator(ui);
                node_breadcrumb(node, package, selected, ui);
            },
            Some(node @ PackageNode::Question { .. }) => {
                breadcrump_separator(ui);
                node_breadcrumb(
                    node.get_parent().unwrap().get_parent().unwrap(),
                    package,
                    selected,
                    ui,
                );
                breadcrump_separator(ui);
                node_breadcrumb(node.get_parent().unwrap(), package, selected, ui);
                breadcrump_separator(ui);
                node_breadcrumb(node, package, selected, ui);
            },
            None => {},
        }
    });
}
