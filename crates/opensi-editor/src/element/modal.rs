use super::danger_button;

/// Ergonomic wrapper around egui's Modal with
/// common style.
pub struct ModalWrapper {
    id: egui::Id,
    inner: egui::Modal,
    open: bool,
}

#[allow(dead_code)]
impl ModalWrapper {
    pub fn new(ctx: &egui::Context, id: impl std::hash::Hash) -> Self {
        let id = egui::Id::new(id);
        let inner = egui::Modal::new(id).frame(egui::Frame::popup(&ctx.style()).inner_margin(20));
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

/// Extension methods for ui for [`ModalWrapper`].
pub trait ModalExt {
    fn close_modal(self);

    fn modal_title(self, title: impl Into<String>) -> egui::Response;

    fn modal_buttons(self, content: impl FnMut(&mut egui::Ui)) -> egui::Response;

    fn modal_danger(self, button: impl Into<egui::WidgetText>) -> egui::Response;

    fn modal_confirm(self, button: impl Into<egui::WidgetText>) -> egui::Response;

    fn modal_button(self, button: impl Into<egui::WidgetText>) -> egui::Response;
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
        let response = self.add(egui::Label::new(
            egui::RichText::new(title)
                .heading()
                .color(self.visuals().widgets.active.bg_stroke.color),
        ));
        self.separator();
        response
    }

    fn modal_buttons(self, content: impl FnMut(&mut egui::Ui)) -> egui::Response {
        self.separator();
        self.with_layout(egui::Layout::right_to_left(egui::Align::Center), content).response
    }

    fn modal_danger(self, button: impl Into<egui::WidgetText>) -> egui::Response {
        let response = danger_button(button, self);
        if response.clicked() {
            self.close_modal();
        }
        response
    }

    fn modal_confirm(self, button: impl Into<egui::WidgetText>) -> egui::Response {
        let response = self
            .scope(|ui| {
                let fg = ui.visuals().widgets.active.bg_stroke.color;
                let bg = fg.linear_multiply(0.1);

                ui.style_mut().visuals.widgets.inactive.fg_stroke.color = fg;
                ui.style_mut().visuals.widgets.active.fg_stroke.color = fg;
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = bg;
                ui.style_mut().visuals.widgets.hovered.bg_fill = bg;
                ui.style_mut().visuals.widgets.hovered.fg_stroke.color = fg;
                ui.add(egui::Button::new(button))
            })
            .inner;
        if response.clicked() {
            self.close_modal();
        }
        response
    }

    fn modal_button(self, button: impl Into<egui::WidgetText>) -> egui::Response {
        let response = self.button(button);
        if response.clicked() {
            self.close_modal();
        }
        response
    }
}
