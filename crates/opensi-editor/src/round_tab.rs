use opensi_core::{Info, Package, PackageNode, Question, Round, Theme};

use crate::utils::{error_label, string_list, unselectable_heading, unselectable_label};

/// Workarea tab to edit round info and its themes.
pub fn round_tab(
    package: &mut Package,
    index: usize,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    ui.vertical(|ui| {
        ui.allocate_ui(egui::vec2(ui.available_width(), 200.0), |ui| {
            egui_extras::StripBuilder::new(ui)
                .sizes(egui_extras::Size::remainder().at_most(500.0), 2)
                .cell_layout(egui::Layout::top_down(egui::Align::Min))
                .horizontal(|mut strip| {
                    let Some(round) = package.get_round_mut(index) else {
                        let error = format!("Невозможно найти раунд с индексом {index}");
                        strip.cell(|ui| {
                            error_label(error, ui);
                        });
                        strip.empty();
                        return;
                    };

                    strip.cell(|ui| {
                        unselectable_heading("Раунд", ui);
                        ui.separator();
                        round_edit(round, ui);
                    });

                    strip.cell(|ui| {
                        unselectable_heading("Дополнительная информация", ui);
                        ui.separator();
                        round_info_edit(&mut round.info, ui);
                    });
                });
        });

        unselectable_heading("Темы", ui);
        ui.separator();
        round_themes(package, index, selected, ui);
    });
}

fn round_edit(round: &mut Round, ui: &mut egui::Ui) {
    egui_extras::TableBuilder::new(ui)
        .id_salt("round-edit")
        .column(egui_extras::Column::auto())
        .column(egui_extras::Column::remainder())
        .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
        .striped(false)
        .body(|mut body| {
            round_edit_row("Название", 20.0, &mut body, |ui| {
                ui.text_edit_singleline(&mut round.name);
            });
            round_edit_row("Вариант", 20.0, &mut body, |ui| {
                // TODO: variant enum
                unselectable_label(format!("TODO: {:?}", round.variant), ui);
            });
        });
}

fn round_info_edit(info: &mut Option<Info>, ui: &mut egui::Ui) {
    let Some(info) = info.as_mut() else {
        if ui.button("➕ Добавить информацию").clicked() {
            *info = Some(Default::default());
        }
        return;
    };

    egui_extras::TableBuilder::new(ui)
        .id_salt("round-info-edit")
        .column(egui_extras::Column::auto())
        .column(egui_extras::Column::remainder())
        .cell_layout(egui::Layout::left_to_right(egui::Align::Min))
        .striped(true)
        .body(|mut body| {
            round_edit_row("Авторы", 40.0, &mut body, |ui| {
                string_list("round-authors", &mut info.authors, ui);
            });
            round_edit_row("Источники", 40.0, &mut body, |ui| {
                string_list("round-sources", &mut info.sources, ui);
            });
            round_edit_row("Комментарий", 20.0, &mut body, |ui| {
                ui.text_edit_singleline(&mut info.comments);
            });
            round_edit_row("Расширения", 20.0, &mut body, |ui| {
                ui.text_edit_singleline(&mut info.extension);
            });
        });
}

fn round_edit_row(
    label: impl AsRef<str>,
    height: f32,
    body: &mut egui_extras::TableBody,
    content: impl FnOnce(&mut egui::Ui),
) {
    body.row(height, |mut row| {
        row.col(|ui| {
            ui.label(label.as_ref());
        });
        row.col(content);
    });
}

fn round_themes(
    package: &mut Package,
    index: usize,
    selected: &mut Option<PackageNode>,
    ui: &mut egui::Ui,
) {
    #[derive(Debug, Clone, Copy)]
    enum CardKind<'a> {
        Theme(&'a Theme),
        Question(&'a Question),
        New,
    }

    fn theme_table_card(kind: CardKind, ui: &mut egui::Ui) -> egui::Response {
        let (text, fill, text_color) = match kind {
            CardKind::Theme(theme) => (
                theme.name.clone(),
                ui.visuals().widgets.active.bg_fill,
                ui.visuals().widgets.active.text_color(),
            ),
            CardKind::Question(question) => (
                question.price.to_string(),
                egui::Color32::TRANSPARENT,
                ui.visuals().widgets.inactive.text_color(),
            ),
            CardKind::New => (
                "➕ Новый вопрос".to_string(),
                egui::Color32::TRANSPARENT,
                ui.visuals().weak_text_color(),
            ),
        };
        let mut frame = egui::Frame::default()
            .inner_margin(16.0)
            .outer_margin(egui::Margin::symmetric(0.0, 4.0))
            .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
            .rounding(8.0)
            .fill(fill)
            .begin(ui);

        // aprox
        let font_size = 22.0 - (text.len() as isize - 8).max(0) as f32 * 0.3;
        frame.content_ui.add_sized(
            egui::vec2(100.0, 0.0),
            egui::Label::new(egui::RichText::new(&text).size(font_size).color(text_color))
                .selectable(false)
                .halign(egui::Align::Center)
                .wrap(),
        );
        let rect =
            frame.content_ui.min_rect() + frame.frame.inner_margin + frame.frame.outer_margin;
        let response = ui.allocate_rect(rect, egui::Sense::click());
        if response.hovered() {
            frame.frame.stroke = ui.style().visuals.widgets.active.bg_stroke;
            frame.frame.fill = ui.style().visuals.widgets.active.weak_bg_fill;
        }
        frame.paint(ui);
        response.on_hover_text(&text)
    }

    egui_extras::StripBuilder::new(ui)
        .size(egui_extras::Size::remainder())
        .size(egui_extras::Size::exact(30.0))
        .cell_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                let Some(round) = package.get_round_mut(index) else {
                    return;
                };

                egui::ScrollArea::both().show(ui, |ui| {
                    let max_theme_len = round
                        .themes
                        .iter()
                        .map(|theme| theme.questions.len())
                        .max()
                        .unwrap_or_default();

                    egui_extras::TableBuilder::new(ui)
                        .id_salt("rounds")
                        .vscroll(false)
                        .columns(egui_extras::Column::remainder(), max_theme_len + 2)
                        .cell_layout(egui::Layout::centered_and_justified(
                            egui::Direction::LeftToRight,
                        ))
                        .body(|body| {
                            body.rows(100.0, round.themes.len(), |mut row| {
                                let theme_index = row.index();
                                let Some(theme) = round.themes.get_mut(theme_index) else {
                                    return;
                                };

                                row.col(|ui| {
                                    if theme_table_card(CardKind::Theme(theme), ui).clicked() {
                                        *selected = Some(PackageNode::Theme {
                                            round_index: index,
                                            index: theme_index,
                                        });
                                    }
                                });

                                for (question_index, question) in theme.questions.iter().enumerate()
                                {
                                    row.col(|ui| {
                                        if theme_table_card(CardKind::Question(question), ui)
                                            .clicked()
                                        {
                                            *selected = Some(PackageNode::Question {
                                                round_index: index,
                                                theme_index,
                                                index: question_index,
                                            });
                                        }
                                    });
                                }

                                row.col(|ui| {
                                    if theme_table_card(CardKind::New, ui).clicked() {
                                        theme.questions.push(Question {
                                            price: theme.guess_next_question_price(),
                                            ..Default::default()
                                        });
                                    }
                                });
                            });
                        });
                });
            });

            strip.cell(|ui| {
                if ui.button("➕ Добавить новую тему").clicked() {
                    package.allocate_theme(index);
                }
            });
        });
}
