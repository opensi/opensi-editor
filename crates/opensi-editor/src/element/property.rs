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
        name: &'static str,
        content: impl FnMut(&mut egui::Ui) -> egui::Response,
    ) {
        self.multiline_row(name, 1, content);
    }

    /// Emplace a new row with named property which takes a few lines.
    pub fn multiline_row(
        &mut self,
        name: &'static str,
        lines: usize,
        mut content: impl FnMut(&mut egui::Ui) -> egui::Response,
    ) {
        self.body.row(self.row_height * lines as f32, |mut row| {
            row.col(|ui| {
                ui.add(egui::Label::new(name).selectable(false));
            });
            row.col(|ui| {
                ui.add_enabled_ui(!self.readonly, |ui| {
                    content(ui);
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
        Self { id, row_height: 22.0, readonly: false }
    }

    pub fn readonly(self, value: bool) -> Self {
        Self { readonly: value, ..self }
    }

    pub fn show(self, ui: &mut egui::Ui, mut builder: impl FnMut(Properties)) {
        egui_extras::TableBuilder::new(ui)
            .id_salt(self.id)
            .column(egui_extras::Column::auto_with_initial_suggestion(150.0))
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
