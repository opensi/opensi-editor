use crate::app::{package_tab, question_tab, round_tab, theme_tab};
use crate::element::node_name;
use crate::icon_string;

use opensi_core::prelude::*;

/// UI for general area of [`Package`] editing.
pub fn workarea(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    egui_extras::StripBuilder::new(ui)
        .size(egui_extras::Size::initial(30.0))
        .size(egui_extras::Size::remainder())
        .cell_layout(egui::Layout::top_down(egui::Align::Min))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                breadcrumbs(package, selected, ui);
            });

            strip.cell(|ui| {
                selected_tab(package, selected, ui);
            });
        });
}

/// Tab ui based on what package node is selected.
fn selected_tab(package: &mut Package, selected: &mut Option<PackageNode>, ui: &mut egui::Ui) {
    match selected {
        &mut Some(PackageNode::Round(idx)) => {
            round_tab::round_tab(package, idx, selected, ui);
        },
        &mut Some(PackageNode::Theme(idx)) => {
            theme_tab::theme_tab(package, idx, selected, ui);
        },
        &mut Some(PackageNode::Question(idx)) => {
            question_tab::question_tab(package, idx, ui);
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
        if breadcrumb(icon_string!(HOUSE, package.name), ui) {
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
            Some(node @ PackageNode::Round(_)) => {
                breadcrump_separator(ui);
                node_breadcrumb(node, package, selected, ui);
            },
            Some(node @ PackageNode::Theme(idx)) => {
                breadcrump_separator(ui);
                node_breadcrumb(idx.parent().into(), package, selected, ui);
                breadcrump_separator(ui);
                node_breadcrumb(node, package, selected, ui);
            },
            Some(node @ PackageNode::Question(idx)) => {
                breadcrump_separator(ui);
                node_breadcrumb(idx.parent().parent().into(), package, selected, ui);
                breadcrump_separator(ui);
                node_breadcrumb(idx.parent().into(), package, selected, ui);
                breadcrump_separator(ui);
                node_breadcrumb(node, package, selected, ui);
            },
            None => {},
        }
    });
}
