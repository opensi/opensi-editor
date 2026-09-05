use crate::app::context::{PackageContext, QuestionContext, RoundContext, ThemeContext};
use crate::app::{package_tab, question_tab, round_tab, theme_tab};
use crate::element::{node_name, panel_header, props::section_label};
use crate::icon_string;

use opensi_core::prelude::*;

/// UI for general area of [`Package`] editing: breadcrumb header + padded body.
/// The vertical scroll lives here, shared by every tab (salted by the selected
/// node so each keeps its own scroll position).
pub fn workarea(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    panel_header(ui, |ui| breadcrumbs(&mut *ctx, ui));
    let m = &crate::style::METRICS;
    let margin = egui::Margin::symmetric((2.0 * m.padding) as i8, (m.padding + m.gap) as i8);
    egui::ScrollArea::vertical().id_salt(ctx.selected()).auto_shrink([false, false]).show(
        ui,
        |ui| {
            egui::Frame::new().inner_margin(margin).show(ui, |ui| {
                ui.set_width(ui.available_width());
                selected_tab(&mut *ctx, ui);
            });
        },
    );
}

/// UI for selected node properties: shared panel header + the node's panel.
pub fn properties(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    let title = match ctx.selected() {
        None => "Параметры Пакета Вопросов",
        Some(PackageNode::Round(_)) => "Параметры Раунда",
        Some(PackageNode::Theme(_)) => "Параметры Темы",
        Some(PackageNode::Question(_)) => "Параметры Вопроса",
    };
    panel_header(ui, |ui| section_label(ui, title));

    // Shared scroll + padded frame for every properties panel.
    egui::ScrollArea::vertical()
        .id_salt(("properties", ctx.selected()))
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Frame::new().inner_margin(crate::style::METRICS.panel_margin()).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 0.0;

                match ctx.selected() {
                    None => package_tab::package_properties(ctx, ui),
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
                }
            });
        });
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

            let text = egui::RichText::new(text.as_ref()).size(crate::style::METRICS.font_body);
            let response = ui
                .add(egui::Label::new(text).extend().sense(egui::Sense::click()).selectable(false));
            response.clicked()
        })
        .inner
    }

    fn breadcrump_separator(ui: &mut egui::Ui) {
        ui.add_space(crate::style::METRICS.gap);
        let text = egui::RichText::new("/").size(crate::style::METRICS.font_label).weak();
        ui.add(egui::Label::new(text).wrap().selectable(false));
        ui.add_space(crate::style::METRICS.gap);
    }

    fn root_breadcrumb(ctx: &mut PackageContext, ui: &mut egui::Ui) {
        let label = icon_string!(HOUSE_SIMPLE, ctx.package().name);
        if breadcrumb(label, ui) {
            ctx.deselect();
        }
    }

    fn node_breadcrumb(ctx: &mut PackageContext, node: PackageNode, ui: &mut egui::Ui) {
        let name = node_name(node, ctx.package());
        if breadcrumb(name, ui) {
            ctx.select(node);
        }
    }

    fn nav_button(ui: &mut egui::Ui, glyph: &str, enabled: bool) -> bool {
        let m = &crate::style::METRICS;
        ui.scope(|ui| {
            let weak = ui.visuals().weak_text_color();
            let w = &mut ui.visuals_mut().widgets;
            w.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            w.inactive.bg_stroke = egui::Stroke::NONE;
            w.inactive.fg_stroke.color = weak;
            w.hovered.bg_stroke = egui::Stroke::NONE;
            w.active.bg_stroke = egui::Stroke::NONE;
            let glyph = egui::RichText::new(glyph).size(m.font_icon);
            ui.add_enabled_ui(enabled, |ui| {
                ui.add_sized(egui::Vec2::splat(m.compact_size), egui::Button::new(glyph))
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
            })
            .inner
        })
        .inner
    }

    let selected = ctx.selected();
    let (prev, next) = selected.map_or((None, None), |node| {
        (ctx.package().prev_node(node), ctx.package().next_node(node))
    });

    root_breadcrumb(ctx, ui);

    match selected {
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

    if selected.is_some() {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = crate::style::METRICS.gap_tiny / 2.0;
            if nav_button(ui, crate::icon!(CARET_RIGHT), next.is_some()) {
                if let Some(node) = next {
                    ctx.select(node);
                }
            }
            if nav_button(ui, crate::icon!(CARET_LEFT), prev.is_some()) {
                if let Some(node) = prev {
                    ctx.select(node);
                }
            }
        });
    }
}
