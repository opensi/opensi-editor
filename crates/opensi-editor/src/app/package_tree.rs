use opensi_core::prelude::*;

use crate::{
    app::context::PackageContext,
    element::{node_context::PackageNodeContextMenu, panel_header, question_is_filled},
    style::{METRICS, mix},
};

/// Ui for a whole [`Package`] in a form of a tree.
pub fn package_tree(ctx: &mut PackageContext, ui: &mut egui::Ui) {
    let total: usize =
        ctx.package().rounds.iter().flat_map(|r| &r.themes).map(|t| t.questions.len()).sum();
    let weak = ui.visuals().weak_text_color();
    let dim = mix(weak, ui.visuals().panel_fill, 0.35);
    panel_header(ui, |ui| {
        ui.add(
            egui::Label::new(
                egui::RichText::new("Пакет Вопросов").size(METRICS.font_body).color(weak),
            )
            .selectable(false),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(total.to_string()).size(METRICS.font_label).color(dim),
                )
                .selectable(false),
            );
        });
    });

    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(METRICS.gap_small as i8, METRICS.gap as i8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = METRICS.gap_tiny / 2.0;
                for r in 0..ctx.package().count_rounds() {
                    tree_node(ctx, PackageNode::Round(r.into()), ui);
                }
            });
    });
}

/// One tree node and, when expanded, its children.
fn tree_node(ctx: &mut PackageContext, node: PackageNode, ui: &mut egui::Ui) {
    let accent = ui.visuals().selection.stroke.color;
    let weak = ui.visuals().weak_text_color();

    let (glyph, glyph_color, label, count, depth, default_open) = match node {
        PackageNode::Round(idx) => {
            let Some(round) = ctx.package().get_round(idx) else { return };
            let count: usize = round.themes.iter().map(|t| t.questions.len()).sum();
            (crate::icon!(SQUARES_FOUR), accent, round.name.clone(), Some(count), 0, true)
        },
        PackageNode::Theme(idx) => {
            let Some(theme) = ctx.package().get_theme(idx) else { return };
            let color = mix(accent, egui::Color32::BLACK, 0.3);
            (crate::icon!(SQUARE), color, theme.name.clone(), Some(theme.questions.len()), 1, false)
        },
        PackageNode::Question(idx) => {
            let Some(question) = ctx.package().get_question(idx) else { return };
            let color = if question_is_filled(question) { accent } else { weak };
            (crate::icon!(QUESTION), color, question.price.to_string(), None, 2, false)
        },
    };

    let id = egui::Id::new(("tree", node));
    let expandable = node.child(0).is_some();
    let mut open = ui.data(|d| d.get_temp(id).unwrap_or(default_open));
    let selected = ctx.selected() == Some(node);
    let count = count.map(|c| c.to_string());

    let (response, toggled) = tree_row(
        ui,
        id,
        depth,
        expandable.then_some(&mut open),
        glyph,
        glyph_color,
        &label,
        count.as_deref(),
        selected,
    );
    ui.data_mut(|d| d.insert_temp(id, open));
    PackageNodeContextMenu { package: ctx.package(), node }.show(&response, ui);
    if response.clicked() && !toggled {
        ctx.select(node);
    }

    if expandable && open {
        let children = match node {
            PackageNode::Round(idx) => ctx.package().count_themes(idx),
            PackageNode::Theme(idx) => ctx.package().count_questions(idx),
            PackageNode::Question(_) => 0,
        };
        for i in 0..children {
            tree_node(ctx, node.child(i).unwrap(), ui);
        }
    }
}

/// A single interactive tree row: disclosure triangle (expandable rows), icon
/// marker, label and a muted trailing count. Returns (row response, whether
/// the triangle was toggled).
fn tree_row(
    ui: &mut egui::Ui,
    id: egui::Id,
    depth: usize,
    toggle: Option<&mut bool>,
    glyph: &str,
    glyph_color: egui::Color32,
    text: &str,
    trailing: Option<&str>,
    selected: bool,
) -> (egui::Response, bool) {
    let m = &METRICS;
    let height = m.compact_size;
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());

    let visuals = ui.visuals();
    let accent = visuals.selection.stroke.color;
    let base = visuals.panel_fill;
    let text_color = visuals.text_color();
    let weak = visuals.weak_text_color();
    let bg = if selected {
        mix(base, accent, 0.20)
    } else if response.hovered() {
        mix(base, text_color, 0.06)
    } else {
        egui::Color32::TRANSPARENT
    };
    let painter = ui.painter();
    if bg != egui::Color32::TRANSPARENT {
        painter.rect_filled(rect, egui::CornerRadius::same(m.rounding), bg);
    }

    let cy = rect.center().y;
    let mut x = rect.left() + m.gap + depth as f32 * m.padding;

    let mut toggled = false;
    let tri_center = egui::pos2(x + m.gap_small, cy);
    if let Some(open) = toggle.as_deref() {
        let glyph = if *open { crate::icon!(CARET_DOWN) } else { crate::icon!(CARET_RIGHT) };
        painter.text(
            tri_center,
            egui::Align2::CENTER_CENTER,
            glyph,
            egui::FontId::proportional(m.font_small),
            weak,
        );
    }
    x += m.padding;

    painter.text(
        egui::pos2(x + m.gap, cy),
        egui::Align2::CENTER_CENTER,
        glyph,
        egui::FontId::proportional(m.font_icon),
        glyph_color,
    );
    x += m.card_padding;

    painter.text(
        egui::pos2(x, cy),
        egui::Align2::LEFT_CENTER,
        text,
        egui::FontId::proportional(m.font_body),
        text_color,
    );

    if let Some(trailing) = trailing {
        painter.text(
            egui::pos2(rect.right() - m.gap, cy),
            egui::Align2::RIGHT_CENTER,
            trailing,
            egui::FontId::proportional(m.font_small),
            weak,
        );
    }

    // Route clicks on the triangle to toggling instead of selection.
    if let Some(open) = toggle {
        let tri_rect = egui::Rect::from_center_size(tri_center, egui::vec2(m.padding, height));
        let tri_response = ui.interact(tri_rect, id.with("toggle"), egui::Sense::click());
        if tri_response.clicked() {
            *open = !*open;
            toggled = true;
        }
    }

    (response, toggled)
}
