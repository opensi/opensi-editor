use opensi_core::prelude::*;
use std::borrow::Cow;

use super::{question_name, round_name, theme_name, unselectable_label};

/// Rectangular cilckable card for package nodes (and more).
// TODO: context menu
#[derive(Debug, Clone, Copy)]
pub struct Card<'a> {
    kind: CardKind<'a>,
    style: CardStyle,
}

/// Types of content of [`Card`].
#[derive(Debug, Clone, Copy)]
pub enum CardKind<'a> {
    Round(&'a Round),
    Theme(&'a Theme),
    Question(&'a Question),
    Custom(&'a str),
}

/// Visual style of [`Card`].
#[derive(Default, Debug, Clone, Copy)]
pub enum CardStyle {
    #[default]
    Normal,
    Important,
    Weak,
}

impl CardStyle {
    pub fn fill_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        match self {
            Self::Important => visuals.widgets.active.weak_bg_fill,
            Self::Normal | Self::Weak => egui::Color32::TRANSPARENT,
        }
    }

    pub fn hover_fill_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        match self {
            Self::Important => visuals.widgets.active.bg_fill,
            Self::Normal | Self::Weak => visuals.widgets.active.weak_bg_fill,
        }
    }

    pub fn text_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        match self {
            Self::Important => visuals.widgets.active.text_color(),
            Self::Weak => visuals.weak_text_color(),
            Self::Normal => visuals.widgets.inactive.text_color(),
        }
    }

    pub fn stroke(&self, visuals: &egui::Visuals) -> egui::Stroke {
        visuals.widgets.noninteractive.bg_stroke
    }

    pub fn hover_stroke(&self, visuals: &egui::Visuals) -> egui::Stroke {
        visuals.widgets.active.bg_stroke
    }
}

impl<'a> egui::Widget for Card<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (fill_color, text_color, stroke) = (
            self.style.fill_color(ui.visuals()),
            self.style.text_color(ui.visuals()),
            self.style.stroke(ui.visuals()),
        );
        let mut frame = egui::Frame::default()
            .inner_margin(16.0)
            .outer_margin(egui::Margin::symmetric(0.0, 4.0))
            .stroke(stroke)
            .rounding(8.0)
            .fill(fill_color)
            .begin(ui);

        let card_width = 120.0;
        let card_height = 80.0;

        frame.content_ui.allocate_ui(egui::vec2(card_width, card_height), |ui| {
            ui.set_min_size(egui::vec2(card_width, card_height));

            let text = match self.kind {
                CardKind::Round(round) => {
                    ui.vertical_centered_justified(|ui| {
                        unselectable_label(
                            egui::RichText::new(round_name(round)).size(22.0).color(text_color),
                            ui,
                        );
                        ui.separator();
                        if round.themes.is_empty() {
                            unselectable_label("Пусто", ui);
                        } else {
                            ui.with_layout(
                                egui::Layout::top_down_justified(egui::Align::Min),
                                |ui| {
                                    for theme in &round.themes {
                                        unselectable_label(format!("⚫ {}", theme_name(theme)), ui);
                                    }
                                },
                            );
                        }
                    });
                    return;
                },
                CardKind::Theme(theme) => theme_name(theme).into(),
                CardKind::Question(question) => question_name(question).into(),
                CardKind::Custom(str) => Cow::Borrowed(str),
            };

            // TODO: aprox, accurate values
            let font_size = 22.0 - (text.len() as isize - 8).max(0) as f32 * 0.3;
            let label = egui::Label::new(
                egui::RichText::new(text.as_ref()).size(font_size).color(text_color),
            )
            .selectable(false)
            .halign(egui::Align::Center)
            .wrap();

            ui.add(label);
        });
        let rect =
            frame.content_ui.min_rect() + frame.frame.inner_margin + frame.frame.outer_margin;
        let response = ui.allocate_rect(rect, egui::Sense::click());
        if response.hovered() {
            frame.frame.stroke = self.style.hover_stroke(ui.visuals());
            frame.frame.fill = self.style.hover_fill_color(ui.visuals());
        }
        frame.paint(ui);
        response
    }
}

/// Builder for a signle row inside [`CardTable`].
pub struct CardTableRow<'a, 'b> {
    row: egui_extras::TableRow<'a, 'b>,
}

impl CardTableRow<'_, '_> {
    pub fn index(&self) -> usize {
        self.row.index()
    }

    fn row(&mut self, mut add: impl FnMut(&mut egui::Ui) -> egui::Response) -> egui::Response {
        let mut response = std::mem::MaybeUninit::uninit();
        self.row.col(|ui| {
            response.write(add(ui));
        });
        unsafe { response.assume_init() }
    }

    pub fn round(&mut self, round: &Round, style: CardStyle) -> egui::Response {
        self.row(|ui| ui.add(Card { kind: CardKind::Round(round), style }))
    }

    pub fn theme(&mut self, theme: &Theme, style: CardStyle) -> egui::Response {
        self.row(|ui| ui.add(Card { kind: CardKind::Theme(theme), style }))
    }

    pub fn question(&mut self, question: &Question, style: CardStyle) -> egui::Response {
        self.row(|ui| ui.add(Card { kind: CardKind::Question(question), style }))
    }

    pub fn custom(&mut self, str: impl AsRef<str>, style: CardStyle) -> egui::Response {
        self.row(|ui| ui.add(Card { kind: CardKind::Custom(str.as_ref()), style }))
    }
}

/// Table for building a grid or list of [`Card`]s.
pub struct CardTable {
    id: egui::Id,
}

impl CardTable {
    pub fn new(id: impl std::hash::Hash) -> Self {
        let id = egui::Id::new(id);
        Self { id }
    }

    pub fn show(
        self,
        ui: &mut egui::Ui,
        count: (usize, usize),
        mut builder: impl FnMut(CardTableRow),
    ) {
        egui_extras::TableBuilder::new(ui)
            .id_salt(self.id)
            .vscroll(false)
            .columns(egui_extras::Column::remainder(), count.0)
            .cell_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight))
            .body(|body| {
                body.rows(120.0, count.1, |row| {
                    let row = CardTableRow { row };
                    builder(row);
                });
            });
    }
}