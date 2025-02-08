use opensi_core::prelude::*;
use std::{borrow::Cow, fmt::Debug};

use super::{
    node_context::PackageNodeContextMenu, question_name, round_name, theme_name, unselectable_label,
};

/// Rectangular cilckable card for package nodes (and more).
// TODO: context menu
#[derive(Debug)]
pub struct Card<'a> {
    kind: CardKind<'a>,
    style: CardStyle,
}

/// Types of content of [`Card`].
#[derive(Debug)]
pub enum CardKind<'a> {
    Round(&'a mut Package, RoundIdx),
    Theme(&'a mut Package, ThemeIdx),
    Question(&'a mut Package, QuestionIdx),
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
            Self::Important | Self::Normal => visuals.widgets.active.bg_fill,
            Self::Weak => egui::Color32::TRANSPARENT,
        }
    }

    pub fn hover_fill_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        match self {
            Self::Important => visuals.widgets.hovered.bg_fill,
            Self::Normal | Self::Weak => visuals.widgets.hovered.weak_bg_fill,
        }
    }

    pub fn text_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        match self {
            Self::Important => visuals.widgets.active.bg_stroke.color,
            Self::Normal => visuals.widgets.inactive.text_color(),
            Self::Weak => visuals.weak_text_color(),
        }
    }

    pub fn hover_text_color(&self, visuals: &egui::Visuals) -> egui::Color32 {
        visuals.widgets.hovered.text_color()
    }

    pub fn stroke(&self, visuals: &egui::Visuals) -> egui::Stroke {
        visuals.widgets.inactive.bg_stroke
    }

    pub fn hover_stroke(&self, visuals: &egui::Visuals) -> egui::Stroke {
        visuals.widgets.hovered.bg_stroke
    }
}

impl Card<'_> {
    pub fn content(&self, ui: &mut egui::Ui, hover: bool) {
        let text_color = if hover {
            self.style.hover_text_color(ui.visuals())
        } else {
            self.style.text_color(ui.visuals())
        };
        let card_width = 160.0;
        let card_height = 80.0;
        let card_size = egui::vec2(card_width, card_height);
        ui.set_min_size(card_size);

        let text = match &self.kind {
            CardKind::Round(package, idx) => {
                let Some(round) = package.get_round(*idx) else {
                    return;
                };
                ui.vertical_centered_justified(|ui| {
                    unselectable_label(
                        egui::RichText::new(round_name(round)).size(22.0).color(text_color),
                        ui,
                    );
                    ui.separator();
                    if round.themes.is_empty() {
                        unselectable_label(egui::RichText::new("Пусто").color(text_color), ui);
                    } else {
                        ui.with_layout(
                            egui::Layout::top_down_justified(egui::Align::Center),
                            |ui| {
                                for theme in &round.themes {
                                    unselectable_label(
                                        egui::RichText::new(theme_name(theme)).color(text_color),
                                        ui,
                                    );
                                }
                            },
                        );
                    }
                });
                return;
            },
            CardKind::Theme(package, idx) => {
                let Some(theme) = package.get_theme(*idx) else {
                    return;
                };
                theme_name(theme).into()
            },
            CardKind::Question(package, idx) => {
                let Some(question) = package.get_question(*idx) else {
                    return;
                };
                question_name(question).into()
            },
            &CardKind::Custom(str) => Cow::Borrowed(str),
        };

        // TODO: aprox, accurate values
        let font_size = 22.0 - (text.len() as isize - 8).max(0) as f32 * 0.3;
        let label =
            egui::Label::new(egui::RichText::new(text.as_ref()).size(font_size).color(text_color))
                .selectable(false)
                .halign(egui::Align::Center)
                .wrap();

        ui.add(label);
    }

    fn context_menu(&mut self, response: &egui::Response, ui: &mut egui::Ui) {
        let (package, node) = match &mut self.kind {
            CardKind::Round(package, round_idx) => (package, (*round_idx).into()),
            CardKind::Theme(package, theme_idx) => (package, (*theme_idx).into()),
            CardKind::Question(package, question_idx) => (package, (*question_idx).into()),
            _ => return,
        };

        PackageNodeContextMenu { package, node }.show(response, ui);
    }
}

impl<'a> egui::Widget for Card<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let (fill_color, stroke) =
            (self.style.fill_color(ui.visuals()), self.style.stroke(ui.visuals()));
        let mut frame = egui::Frame::default()
            .inner_margin(16.0)
            .outer_margin(egui::Margin::symmetric(0.0, 4.0))
            .stroke(stroke)
            .rounding(8.0)
            .fill(fill_color)
            .begin(ui);

        // yucky two pass solution to hover cards bleh
        let hover = {
            let builder = egui::UiBuilder::new();
            let mut sizing_ui = frame.content_ui.new_child(builder);
            self.content(&mut sizing_ui, false);
            let rect = sizing_ui.min_rect() + frame.frame.inner_margin + frame.frame.outer_margin;
            sizing_ui.rect_contains_pointer(rect)
        };

        self.content(&mut frame.content_ui, hover);
        let rect =
            frame.content_ui.min_rect() + frame.frame.inner_margin + frame.frame.outer_margin;
        if hover {
            frame.frame.stroke = self.style.hover_stroke(ui.visuals());
            frame.frame.fill = self.style.hover_fill_color(ui.visuals());
        }
        frame.paint(ui);

        let response = ui.allocate_rect(rect, egui::Sense::click());
        self.context_menu(&response, ui);
        response
    }
}

/// Builder for a signle row inside [`CardTable`].
pub struct CardTableRow<'a, 'b> {
    strip: egui_extras::Strip<'a, 'b>,
    index: usize,
}

impl CardTableRow<'_, '_> {
    pub fn index(&self) -> usize {
        self.index
    }

    fn row(&mut self, mut add: impl FnMut(&mut egui::Ui) -> egui::Response) -> egui::Response {
        let mut response = std::mem::MaybeUninit::uninit();
        self.strip.cell(|ui| {
            response.write(add(ui));
        });
        unsafe { response.assume_init() }
    }

    pub fn round(
        &mut self,
        package: &mut Package,
        idx: impl Into<RoundIdx>,
        style: CardStyle,
    ) -> egui::Response {
        let idx = idx.into();
        self.row(|ui| ui.add(Card { kind: CardKind::Round(package, idx), style }))
    }

    pub fn theme(
        &mut self,
        package: &mut Package,
        idx: impl Into<ThemeIdx>,
        style: CardStyle,
    ) -> egui::Response {
        let idx = idx.into();
        self.row(|ui| ui.add(Card { kind: CardKind::Theme(package, idx), style }))
    }

    pub fn question(
        &mut self,
        package: &mut Package,
        idx: impl Into<QuestionIdx>,
        style: CardStyle,
    ) -> egui::Response {
        let idx = idx.into();
        self.row(|ui| ui.add(Card { kind: CardKind::Question(package, idx), style }))
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
        egui::ScrollArea::both()
            .id_salt(self.id)
            // FIXME: this fixes always present horizontal scroll, but kinda yucky fix
            .min_scrolled_width(ui.available_width() + 1.0)
            // .max_height(ui.max_rect().height())
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    egui_extras::StripBuilder::new(ui)
                        .sizes(egui_extras::Size::initial(80.0), count.1)
                        .vertical(|mut vertical| {
                            for row in 0..count.1 {
                                vertical.strip(|horizontal| {
                                    horizontal
                                        .sizes(egui_extras::Size::remainder(), count.0)
                                        .cell_layout(egui::Layout::centered_and_justified(
                                            egui::Direction::LeftToRight,
                                        ))
                                        .horizontal(|strip| {
                                            let row = CardTableRow { strip, index: row };
                                            builder(row);
                                        });
                                });
                            }
                        });
                });
            });
    }
}
