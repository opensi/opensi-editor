use crate::app::FONT_BOLD_ID;

/// Row builder for [`PropertyTable`].
pub struct Properties<'t> {
    row_height: f32,
    readonly: bool,
    body: egui_extras::TableBody<'t>,
}

impl<'t> Properties<'t> {
    /// Emplace a new row with named property.
    pub fn row(
        &mut self,
        icon: &'static str,
        name: &'static str,
        content: impl FnMut(&mut egui::Ui) -> egui::Response,
    ) {
        self.multiline_row(icon, name, 1, content);
    }

    /// Emplace a new row with named property which takes a few lines.
    pub fn multiline_row(
        &mut self,
        icon: &'static str,
        name: &'static str,
        lines: usize,
        mut content: impl FnMut(&mut egui::Ui) -> egui::Response,
    ) {
        self.body.row(self.row_height * lines as f32, |mut row| {
            row.col(|ui| {
                ui.add(
                    egui::Label::new(egui::RichText::new(icon).size(26.0))
                        .halign(egui::Align::RIGHT)
                        .selectable(false),
                );
            });
            row.col(|ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin { left: 0, right: 4, top: 4, bottom: 4 })
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(2.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(name)
                                            .strong()
                                            .size(16.0)
                                            .family(egui::FontFamily::Name(FONT_BOLD_ID.into())),
                                    )
                                    .selectable(false),
                                );
                            });
                            ui.add_enabled_ui(!self.readonly, |ui| {
                                content(ui);
                            });
                        });
                    });
            });
        });
    }
}

/// Table with two columns: property names, and property values.
pub struct PropertyTable {
    id: egui::Id,
    row_height: f32,
    readonly: bool,
}

impl PropertyTable {
    pub fn new(id: impl std::hash::Hash) -> Self {
        let id = egui::Id::new(id);
        Self { id, row_height: 40.0, readonly: false }
    }

    pub fn readonly(self, value: bool) -> Self {
        Self { readonly: value, ..self }
    }

    pub fn show(self, ui: &mut egui::Ui, mut builder: impl FnMut(Properties)) {
        egui_extras::TableBuilder::new(ui)
            .id_salt(self.id)
            .column(egui_extras::Column::exact(26.0))
            .column(egui_extras::Column::remainder())
            .cell_layout(
                egui::Layout::left_to_right(egui::Align::Center)
                    .with_main_justify(true)
                    .with_main_align(egui::Align::Min),
            )
            .vscroll(false)
            .body(|body| {
                let properties =
                    Properties { row_height: self.row_height, readonly: self.readonly, body };
                builder(properties);
            });
    }
}
