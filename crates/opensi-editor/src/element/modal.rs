use crate::{element::common::outlined_button, style};

/// Ergonomic wrapper around egui's Modal with common style.
pub struct ModalWrapper {
    id: egui::Id,
    inner: egui::Modal,
    open: bool,
}

#[allow(dead_code)]
impl ModalWrapper {
    pub fn new(ctx: &egui::Context, id: impl std::hash::Hash) -> Self {
        let id = egui::Id::new(id);
        let border = style::current_scheme(ctx).border();
        let inner = egui::Modal::new(id).frame(
            egui::Frame::new()
                .fill(ctx.style().visuals.window_fill)
                .stroke(egui::Stroke::new(1.0_f32, border))
                .corner_radius(style::METRICS.rounding_large)
                .shadow(ctx.style().visuals.window_shadow)
                .inner_margin(egui::Margin::ZERO),
        );
        let open = ctx.memory(|memory| memory.data.get_temp(id.with("open"))).unwrap_or_default();
        Self { inner, id, open }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    pub fn open(&mut self) {
        self.set_open(true);
    }

    pub fn close(&mut self) {
        self.set_open(false);
    }

    pub fn show(self, ctx: &egui::Context, content: impl FnMut(&mut egui::Ui)) {
        let open_id = self.id.with("open");
        if self.is_open() {
            ctx.memory_mut(|memory| {
                memory.data.insert_temp(egui::Id::new("last-modal-id"), open_id);
                if memory.data.get_temp::<bool>(open_id).is_none_or(|open| !open) {
                    memory.data.insert_temp(open_id, true);
                }
            });
            let response = self.inner.show(ctx, content);
            if response.should_close() {
                ctx.memory_mut(|memory| memory.data.insert_temp(open_id, false));
            }
        }
    }
}

fn modal_section(
    ui: &mut egui::Ui,
    pad_y: i8,
    top_divider: bool,
    bottom_divider: bool,
    add: impl FnOnce(&mut egui::Ui),
) -> egui::Response {
    let pad_x = 2.0 * style::METRICS.padding;
    let margin = egui::Margin { left: pad_x as i8, right: pad_x as i8, top: pad_y, bottom: pad_y };
    let inner_w = style::METRICS.modal_width - 2.0 * pad_x;
    let resp = egui::Frame::new()
        .inner_margin(margin)
        .show(ui, |ui| {
            ui.set_width(inner_w);
            add(ui);
        })
        .response;
    let border = style::current_scheme(ui.ctx()).border();
    if top_divider {
        ui.painter().hline(
            resp.rect.x_range(),
            resp.rect.top(),
            egui::Stroke::new(1.0_f32, border),
        );
    }
    if bottom_divider {
        ui.painter().hline(
            resp.rect.x_range(),
            resp.rect.bottom(),
            egui::Stroke::new(1.0_f32, border),
        );
    }
    resp
}

/// Extension methods for ui for [`ModalWrapper`].
pub trait ModalExt {
    fn close_modal(self);

    /// A padded title section (heading) with a divider below it.
    fn modal_title(self, title: impl Into<String>) -> egui::Response;

    /// A title section with a muted description line under the heading.
    fn modal_title_desc(self, title: impl Into<String>, description: &str) -> egui::Response;

    /// A padded content section (no dividers) for the modal body.
    fn modal_body(self, content: impl FnOnce(&mut egui::Ui)) -> egui::Response;

    /// A padded, right-aligned button row with a divider above it.
    fn modal_buttons(self, content: impl FnMut(&mut egui::Ui)) -> egui::Response;

    fn modal_danger(self, button: &str) -> egui::Response;

    fn modal_confirm(self, button: &str) -> egui::Response;
}

impl ModalExt for &'_ mut egui::Ui {
    fn close_modal(self) {
        let Some(open_id) =
            self.memory(|memory| memory.data.get_temp(egui::Id::new("last-modal-id")))
        else {
            return;
        };
        self.memory_mut(|memory| memory.data.insert_temp(open_id, false));
    }

    fn modal_title(self, title: impl Into<String>) -> egui::Response {
        self.modal_title_desc(title, "")
    }

    fn modal_title_desc(self, title: impl Into<String>, description: &str) -> egui::Response {
        let color = self.visuals().text_color();
        let weak = self.visuals().weak_text_color();
        modal_section(self, style::METRICS.card_padding as i8, false, true, |ui| {
            ui.spacing_mut().item_spacing.y = style::METRICS.gap_small;
            ui.add(
                egui::Label::new(egui::RichText::new(title).heading().color(color))
                    .selectable(false),
            );
            if !description.is_empty() {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(description).size(style::METRICS.font_body).color(weak),
                    )
                    .selectable(false),
                );
            }
        })
    }

    fn modal_body(self, content: impl FnOnce(&mut egui::Ui)) -> egui::Response {
        modal_section(self, style::METRICS.card_padding as i8, false, false, content)
    }

    fn modal_buttons(self, content: impl FnMut(&mut egui::Ui)) -> egui::Response {
        modal_section(self, style::METRICS.padding as i8, true, false, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), content);
        })
    }

    fn modal_danger(self, button: &str) -> egui::Response {
        let color = self.visuals().error_fg_color;
        let response = modal_action_button(self, button, color);
        if response.clicked() {
            self.close_modal();
        }
        response
    }

    fn modal_confirm(self, button: &str) -> egui::Response {
        let color = self.visuals().selection.stroke.color;
        let response = modal_action_button(self, button, color);
        if response.clicked() {
            self.close_modal();
        }
        response
    }
}

/// A content-sized [`outlined_button`] for the modal's action row.
fn modal_action_button(ui: &mut egui::Ui, label: &str, color: egui::Color32) -> egui::Response {
    let width = ui
        .fonts(|f| {
            f.layout_no_wrap(
                label.to_owned(),
                egui::FontId::proportional(style::METRICS.font_label),
                color,
            )
        })
        .size()
        .x;
    outlined_button(
        ui,
        egui::vec2(width + 2.0 * style::METRICS.padding, style::METRICS.button_height),
        label,
        color,
    )
}
