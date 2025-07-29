use core::panic;

use super::unselectable_heading;

pub struct SectionLine<'a, 'b> {
    current: usize,
    line_index: usize,
    len: u8,
    line_strip: egui_extras::Strip<'a, 'b>,
}

impl SectionLine<'_, '_> {
    pub fn section(&mut self, name: impl AsRef<str>, mut add_contents: impl FnMut(&mut egui::Ui)) {
        if self.current >= self.len as usize {
            panic!("Selection is over line limit");
        }

        let margin = 20;
        let left_margin = if self.current == 0 { 0 } else { margin };
        let right_margin = if self.current == (self.len as usize - 1) { 0 } else { margin };

        self.line_strip.cell(|ui| {
            ui.push_id(
                ui.id().with(format!("section-{}-{}", self.line_index, self.current)),
                |ui| {
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: left_margin,
                            right: right_margin,
                            top: 0,
                            bottom: 0,
                        })
                        .show(ui, |ui| {
                            unselectable_heading(name.as_ref(), ui);
                            ui.separator();
                            add_contents(ui);
                        });
                },
            );
        });
        self.current += 1;
    }
}

pub struct SectionsBody<'s, 'a, 'b> {
    current: usize,
    lines: &'s [u8],
    body_strip: egui_extras::Strip<'a, 'b>,
}

impl SectionsBody<'_, '_, '_> {
    pub fn line(&mut self, mut add_contents: impl FnMut(SectionLine)) {
        let len = self.lines[self.current];
        self.body_strip.strip(|builder| {
            builder
                .sizes(egui_extras::Size::remainder(), len as usize)
                .cell_layout(egui::Layout::top_down(egui::Align::Min))
                .clip(true)
                .horizontal(|strip| {
                    let line = SectionLine {
                        len,
                        current: 0,
                        line_index: self.current,
                        line_strip: strip,
                    };
                    add_contents(line);
                });
        });
        self.current += 1;
    }
}

#[derive(Debug)]
pub struct Sections {
    id: egui::Id,
    lines: Vec<u8>,
    sizes: Vec<egui_extras::Size>,
}

impl Sections {
    pub fn new(id: impl std::hash::Hash) -> Self {
        let id = egui::Id::new(id);
        Self { id, lines: vec![], sizes: vec![] }
    }

    pub fn line(mut self, size: egui_extras::Size, sections: u8) -> Self {
        self.lines.push(sections);
        self.sizes.push(size);
        self
    }

    pub fn show(self, ui: &mut egui::Ui, mut add_contents: impl FnMut(SectionsBody)) {
        ui.push_id(self.id, |ui| {
            let mut builder = egui_extras::StripBuilder::new(ui)
                .clip(true)
                .cell_layout(egui::Layout::top_down_justified(egui::Align::Min));
            for size in self.sizes {
                builder = builder.size(size);
            }

            builder.vertical(|strip| {
                let body =
                    SectionsBody { lines: self.lines.as_slice(), current: 0, body_strip: strip };
                add_contents(body);
            });
        });
    }
}
