pub mod themes;

/// Shared layout metrics (paddings, gaps, sizes) used across the editor UI, so
/// panels and widgets agree on spacing instead of hardcoding it per call site.
pub struct Metrics {
    // -- Chrome --------------------------------------------------------------
    /// Top navbar height.
    pub navbar_height: f32,
    /// Panel header row height.
    pub header_height: f32,

    // -- Spacing -------------------------------------------------------------
    /// Standard content padding; also the gap between sections and fields, and
    /// the tree indent per depth level.
    pub padding: f32,
    /// Small padding / gap: row gaps in lists and grids, inner card padding.
    pub padding_small: f32,
    /// Standard gap between items.
    pub gap: f32,
    /// Tight gap between closely related items; also the inline scrollbar width.
    pub gap_small: f32,
    /// Minimal gap: micro spacing inside dense rows and lists.
    pub gap_tiny: f32,
    /// Hairline width: dividers, thin bar rounding, menu row spacing.
    pub hairline: f32,

    // -- Rounding ------------------------------------------------------------
    /// Corner radius of cards, cells, chips and buttons.
    pub rounding: u8,
    /// Corner radius of large surfaces (modal card, hero buttons).
    pub rounding_large: u8,

    // -- Sizes ---------------------------------------------------------------
    /// Width of the accent bar on cards / cells / rows.
    pub accent_bar_width: f32,
    /// Height of action buttons and option rows.
    pub button_height: f32,
    /// Side of small square icon buttons; also the compact (tree) row height.
    pub compact_size: f32,
    /// Height of full-width "add" rows and the hero buttons.
    pub add_row_height: f32,
    /// Inner padding of large cards; also the hero button label padding.
    pub card_padding: f32,

    // -- Modal / menu --------------------------------------------------------
    /// Fixed content width of every modal card.
    pub modal_width: f32,
    /// Fixed content width of dropdown / context menus.
    pub menu_width: f32,
    /// Height of one menu item row.
    pub menu_row_height: f32,

    // -- Grid / list ---------------------------------------------------------
    /// Height of one theme row in the round grid.
    pub grid_row_height: f32,
    /// Width of a question price cell; also the theme list's price column.
    pub grid_cell_width: f32,
    /// Width of the trailing "add question" cell.
    pub grid_add_width: f32,
    /// Max width of the theme name cell.
    pub grid_theme_width: f32,
    /// Height of one question row in the theme editor list.
    pub list_row_height: f32,

    // -- Welcome hero --------------------------------------------------------
    /// Side of the welcome page's accent logo badge.
    pub hero_badge_size: f32,
    /// Estimated height of the hero block (used to center it vertically).
    pub hero_height: f32,

    // -- Fonts ---------------------------------------------------------------
    /// Small print: meta lines, counts, section labels.
    pub font_small: f32,
    /// Field labels, chips, hints and button labels.
    pub font_label: f32,
    /// Body text: inputs, tree labels, card titles, menu items.
    pub font_body: f32,
    /// Icon glyphs and regular button text.
    pub font_icon: f32,
    /// Medium accents: header button glyphs, the welcome tagline.
    pub font_medium: f32,
    /// Large accents: prices, the navbar logo glyph.
    pub font_large: f32,
    /// Big headings (round card title).
    pub font_heading: f32,
    /// Display size (welcome title and badge glyph).
    pub font_display: f32,
}

impl Metrics {
    /// Standard panel margin ([`Metrics::padding`] on all sides).
    pub fn panel_margin(&self) -> egui::Margin {
        egui::Margin::same(self.padding as i8)
    }

    /// Inner margin of large cards ([`Metrics::card_padding`] on all sides).
    pub fn card_margin(&self) -> egui::Margin {
        egui::Margin::same(self.card_padding as i8)
    }
}

/// The one set of metrics the whole UI is designed around.
pub static METRICS: Metrics = Metrics {
    navbar_height: 44.0,
    header_height: 38.0,
    padding: 16.0,
    padding_small: 12.0,
    gap: 8.0,
    gap_small: 6.0,
    gap_tiny: 4.0,
    hairline: 1.0,
    rounding: 4,
    rounding_large: 6,
    accent_bar_width: 2.0,
    button_height: 36.0,
    compact_size: 26.0,
    add_row_height: 48.0,
    card_padding: 20.0,
    modal_width: 500.0,
    menu_width: 258.0,
    menu_row_height: 30.0,
    grid_row_height: 74.0,
    grid_cell_width: 78.0,
    grid_add_width: 120.0,
    grid_theme_width: 220.0,
    list_row_height: 62.0,
    hero_badge_size: 64.0,
    hero_height: 300.0,
    font_small: 11.0,
    font_label: 12.0,
    font_body: 13.0,
    font_icon: 14.0,
    font_medium: 15.0,
    font_large: 17.0,
    font_heading: 20.0,
    font_display: 32.0,
};

const DEFAULT_FONT_STYLE: FontStyle =
    FontStyle { heading_size: 20.0, regular_size: 14.0, button_size: 14.0, small_size: 12.0 };

pub fn all_themes() -> impl Iterator<Item = &'static dyn AppTheme> {
    [
        &themes::OpenSIBlue as &dyn AppTheme,
        &themes::OpenSIGreen as &dyn AppTheme,
        &themes::OpenSIOrange as &dyn AppTheme,
        &themes::OpenSIPeach as &dyn AppTheme,
    ]
    .into_iter()
}

pub fn choose(name: impl AsRef<str>) -> Option<&'static dyn AppTheme> {
    let name = name.as_ref();
    all_themes().find(|theme| theme.name() == name)
}

pub fn default_theme() -> &'static dyn AppTheme {
    all_themes().next().unwrap()
}

fn active_scheme_id() -> egui::Id {
    egui::Id::new("opensi-active-color-scheme")
}

/// The [`ColorScheme`] of the currently-applied theme
pub fn current_scheme(ctx: &egui::Context) -> ColorScheme {
    ctx.data(|d| d.get_temp(active_scheme_id()))
        .unwrap_or_else(|| default_theme().colorscheme(ColorMode::default()).clone())
}

/// Linear blend of two colors, `t` from `a` (0.0) to `b` (1.0)
pub fn mix(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let c = |x: u8, y: u8| (x as f32 * (1.0 - t) + y as f32 * t).round() as u8;
    egui::Color32::from_rgb(c(a.r(), b.r()), c(a.g(), b.g()), c(a.b(), b.b()))
}

/// Which of a theme's two color-scheme variants is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorMode {
    Day,
    Night,
}

impl ColorMode {
    /// The phosphor glyph representing this mode (sun / moon).
    pub fn glyph(&self) -> &'static str {
        match self {
            ColorMode::Day => crate::icon!(SUN),
            ColorMode::Night => crate::icon!(MOON),
        }
    }
}

impl std::fmt::Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            ColorMode::Day => "День",
            ColorMode::Night => "Ночь",
        };
        f.write_str(label)
    }
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Night
    }
}

pub trait AppTheme {
    /// Light (day) variant of this theme's colors.
    fn colorscheme_day(&self) -> &ColorScheme;
    /// Dark (night) variant of this theme's colors.
    fn colorscheme_night(&self) -> &ColorScheme;
    fn name(&self) -> &'static str;

    /// The color scheme for the given [`ColorMode`].
    fn colorscheme(&self, mode: ColorMode) -> &ColorScheme {
        match mode {
            ColorMode::Day => self.colorscheme_day(),
            ColorMode::Night => self.colorscheme_night(),
        }
    }

    fn font_style(&self) -> &FontStyle {
        &DEFAULT_FONT_STYLE
    }

    fn apply(&self, ctx: &egui::Context, mode: ColorMode) {
        self.colorscheme(mode).apply(ctx);
        self.font_style().apply(ctx);
    }
}

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub text: egui::Color32,
    pub text_strong: egui::Color32,
    pub text_weak: egui::Color32,
    pub base: egui::Color32,
    pub base_weak: egui::Color32,
    pub base_alt: egui::Color32,
    pub base_strong: egui::Color32,
    pub accent: egui::Color32,
    pub link: egui::Color32,
    pub warn: egui::Color32,
    pub error: egui::Color32,
    pub dark: bool,
}

impl ColorScheme {
    pub fn border(&self) -> egui::Color32 {
        mix(self.base, self.text_weak, 0.22)
    }

    pub fn border_strong(&self) -> egui::Color32 {
        mix(self.base, self.text_weak, 0.4)
    }

    pub fn hover_fill(&self) -> egui::Color32 {
        mix(self.base_alt, self.text, 0.10)
    }

    pub fn apply(&self, ctx: &egui::Context) {
        ctx.data_mut(|d| d.insert_temp(active_scheme_id(), self.clone()));

        ctx.all_styles_mut(|style| {
            style.spacing.menu_margin = egui::Margin::same(5);
            style.spacing.menu_spacing = 0.0;
            style.spacing.button_padding = egui::vec2(10.0, 6.0);
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);

            let visuals = &mut style.visuals;
            let accent_bg = self.accent.linear_multiply(0.2);

            let border = self.border();
            let border_strong = self.border_strong();
            let hover_fill = self.hover_fill();
            let active_fill = mix(self.base_alt, self.text, 0.16);
            let input_bg = mix(self.base, self.text, 0.02);
            let radius = egui::CornerRadius::same(5);

            visuals.hyperlink_color = self.link;
            visuals.warn_fg_color = self.warn;
            visuals.error_fg_color = self.error;

            visuals.window_fill = input_bg;
            visuals.panel_fill = self.base;
            visuals.menu_corner_radius = egui::CornerRadius::same(0);
            visuals.faint_bg_color = self.base_weak;
            visuals.window_stroke = egui::Stroke::new(1.0_f32, border);
            visuals.extreme_bg_color = input_bg;
            visuals.code_bg_color = self.base_strong;

            visuals.selection.bg_fill = accent_bg;
            visuals.selection.stroke = egui::Stroke::new(1.0_f32, self.accent);

            visuals.text_cursor = egui::style::TextCursorStyle {
                stroke: egui::Stroke::new(2.0_f32, self.accent),
                preview: false,
                blink: true,
                on_duration: 0.66,
                off_duration: 0.33,
            };

            visuals.widgets.noninteractive = egui::style::WidgetVisuals {
                bg_fill: self.base,
                weak_bg_fill: self.base_weak,
                bg_stroke: egui::Stroke::new(1.0_f32, border),
                fg_stroke: egui::Stroke::new(1.0_f32, self.text_weak),
                corner_radius: radius,
                expansion: 0.0,
            };
            visuals.widgets.inactive = egui::style::WidgetVisuals {
                bg_fill: self.base_alt,
                weak_bg_fill: self.base_alt,
                bg_stroke: egui::Stroke::new(1.0_f32, border),
                fg_stroke: egui::Stroke::new(1.0_f32, self.text),
                corner_radius: radius,
                expansion: 0.0,
            };
            visuals.widgets.hovered = egui::style::WidgetVisuals {
                bg_fill: hover_fill,
                weak_bg_fill: hover_fill,
                bg_stroke: egui::Stroke::new(1.0_f32, border_strong),
                fg_stroke: egui::Stroke::new(1.0_f32, self.text_strong),
                corner_radius: radius,
                expansion: 0.0,
            };
            visuals.widgets.active = egui::style::WidgetVisuals {
                bg_fill: active_fill,
                weak_bg_fill: active_fill,
                bg_stroke: egui::Stroke::new(1.0_f32, border_strong),
                fg_stroke: egui::Stroke::new(1.0_f32, self.text_strong),
                corner_radius: radius,
                expansion: 0.0,
            };
            visuals.widgets.open = egui::style::WidgetVisuals {
                bg_stroke: egui::Stroke::new(1.0_f32, border_strong),
                ..visuals.widgets.active.clone()
            };

            visuals.window_shadow = egui::Shadow {
                offset: [0, 5],
                blur: 10,
                spread: 0,
                color: egui::Color32::from_black_alpha(80),
            };
            visuals.popup_shadow = egui::Shadow {
                offset: [0, 8],
                blur: 16,
                spread: 0,
                color: egui::Color32::from_black_alpha(90),
            };

            visuals.dark_mode = self.dark;
        });
    }
}

#[derive(Debug, Clone)]
pub struct FontStyle {
    pub heading_size: f32,
    pub regular_size: f32,
    pub button_size: f32,
    pub small_size: f32,
}

impl Default for FontStyle {
    fn default() -> Self {
        DEFAULT_FONT_STYLE.clone()
    }
}

impl FontStyle {
    pub fn apply(&self, ctx: &egui::Context) {
        ctx.all_styles_mut(|style| {
            style.text_styles = [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(self.heading_size, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(self.regular_size, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(self.regular_size, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(self.button_size, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(self.small_size, egui::FontFamily::Proportional),
                ),
            ]
            .into();
        });
    }
}
