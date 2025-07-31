use crate::app::context::{PackageContext, QuestionContext, RoundContext, ThemeContext};
use crate::app::{package_tab, question_tab, round_tab, theme_tab};
use crate::element::node_name;
use crate::icon_string;

use opensi_core::prelude::*;

/// UI for general area of [`Package`] editing.
pub fn workarea(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    egui_extras::StripBuilder::new(ui)
        .size(egui_extras::Size::initial(40.0))
        .size(egui_extras::Size::remainder())
        .cell_layout(egui::Layout::top_down(egui::Align::Min))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                breadcrumbs(ctx, ui);
            });

            strip.cell(|ui| {
                selected_tab(ctx, ui);
            });
        });
}

/// UI for selected node properties.
pub fn properties(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    match ctx.selected() {
        Some(PackageNode::Round(idx)) => {
            if let Some(mut ctx) = RoundContext::try_new(ctx, idx) {
                round_tab::round_properties(&mut ctx, ui);
            }
        },
        Some(PackageNode::Theme(idx)) => {
            if let Some(mut ctx) = ThemeContext::try_new(ctx, idx) {
                theme_tab::theme_properties(&mut ctx, ui);
            }
        },
        Some(PackageNode::Question(idx)) => {
            if let Some(mut ctx) = QuestionContext::try_new(ctx, idx) {
                question_tab::question_properties(&mut ctx, ui);
            }
        },
        None => {
            package_tab::package_properties(ctx, ui);
        },
    }
}

/// Tab ui based on what package node is selected.
fn selected_tab(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    match ctx.selected() {
        Some(PackageNode::Round(idx)) => {
            if let Some(mut ctx) = RoundContext::try_new(ctx, idx) {
                round_tab::round_tab(&mut ctx, ui);
            }
        },
        Some(PackageNode::Theme(idx)) => {
            if let Some(mut ctx) = ThemeContext::try_new(ctx, idx) {
                theme_tab::theme_tab(&mut ctx, ui);
            }
        },
        Some(PackageNode::Question(idx)) => {
            if let Some(mut ctx) = QuestionContext::try_new(ctx, idx) {
                question_tab::question_tab(&mut ctx, ui);
            }
        },
        None => {
            package_tab::package_tab(ctx, ui);
        },
    }
}

/// Selection breadcrumbs ui.
fn breadcrumbs(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    fn breadcrumb(text: impl AsRef<str>, ui: &mut egui::Ui) -> bool {
        ui.scope(|ui| {
            ui.visuals_mut().widgets.hovered.fg_stroke.color = ui.visuals().text_color();
            ui.visuals_mut().widgets.inactive.fg_stroke.color = ui.visuals().weak_text_color();

            let text = egui::RichText::new(text.as_ref()).size(18.0);
            let response = ui
                .add(egui::Label::new(text).extend().sense(egui::Sense::click()).selectable(false));
            response.clicked()
        })
        .inner
    }

    fn breadcrump_separator(ui: &mut egui::Ui) {
        ui.add_space(8.0);
        let text = egui::RichText::new("/").size(14.0).weak();
        ui.add(egui::Label::new(text).wrap().selectable(false));
        ui.add_space(8.0);
    }

    fn root_breadcrumb(ctx: &mut PackageContext, ui: &mut egui::Ui) {
        if breadcrumb(icon_string!(HOUSE, ctx.package().name), ui) {
            ctx.deselect();
        }
    }

    fn node_breadcrumb(ctx: &mut PackageContext, node: PackageNode, ui: &mut egui::Ui) {
        let name = node_name(node, ctx.package());
        if breadcrumb(name, ui) {
            ctx.select(node);
        }
    }

    ui.horizontal(|ui| {
        root_breadcrumb(ctx, ui);

        match ctx.selected() {
            Some(node @ PackageNode::Round(_)) => {
                breadcrump_separator(ui);
                node_breadcrumb(ctx, node, ui);
            },
            Some(node @ PackageNode::Theme(idx)) => {
                breadcrump_separator(ui);
                node_breadcrumb(ctx, idx.parent().into(), ui);
                breadcrump_separator(ui);
                node_breadcrumb(ctx, node, ui);
            },
            Some(node @ PackageNode::Question(idx)) => {
                breadcrump_separator(ui);
                node_breadcrumb(ctx, idx.parent().parent().into(), ui);
                breadcrump_separator(ui);
                node_breadcrumb(ctx, idx.parent().into(), ui);
                breadcrump_separator(ui);
                node_breadcrumb(ctx, node, ui);
            },
            None => {},
        }
    });
}
