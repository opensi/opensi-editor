pub mod catppuccin;

const DEFAULT_FONT_STYLE: FontStyle =
    FontStyle { heading_size: 20.0, regular_size: 14.0, button_size: 14.0, small_size: 12.0 };

pub fn all_themes() -> impl Iterator<Item = &'static dyn AppTheme> {
    [&catppuccin::Mocha as &dyn AppTheme, &catppuccin::Latte as &dyn AppTheme].into_iter()
}

pub fn choose(name: impl AsRef<str>) -> Option<&'static dyn AppTheme> {
    let name = name.as_ref();
    all_themes().find(|theme| theme.name() == name)
}

pub fn default_theme() -> &'static dyn AppTheme {
    all_themes().next().unwrap()
}

pub trait AppTheme {
    fn color_scheme(&self) -> &ColorScheme;
    fn name(&self) -> &'static str;

    fn font_style(&self) -> &FontStyle {
        &DEFAULT_FONT_STYLE
    }

    fn apply(&self, ctx: &egui::Context) {
        self.color_scheme().apply(ctx);
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
    pub fn apply(&self, ctx: &egui::Context) {
        ctx.style_mut(|style| {
            style.spacing.menu_margin = egui::Margin::same(10);
            style.spacing.button_padding = egui::vec2(4.0, 2.0);

            let visuals = &mut style.visuals;
            let accent_bg = self.accent.linear_multiply(0.2);
            let accent_bg_weak = self.accent.linear_multiply(0.1);

            visuals.hyperlink_color = self.link;
            visuals.warn_fg_color = self.warn;
            visuals.error_fg_color = self.error;

            visuals.window_fill = self.base;
            visuals.panel_fill = self.base;
            visuals.faint_bg_color = self.base_weak;
            visuals.window_stroke.color = self.base;
            visuals.extreme_bg_color = self.base_strong;
            visuals.code_bg_color = self.base_strong;

            visuals.selection.bg_fill = accent_bg;
            visuals.selection.stroke = egui::Stroke::new(1.0, self.accent);

            visuals.text_cursor = egui::style::TextCursorStyle {
                stroke: egui::Stroke::new(2.0, self.accent),
                preview: false,
                blink: true,
                on_duration: 0.66,
                off_duration: 0.33,
            };

            visuals.widgets.noninteractive = egui::style::WidgetVisuals {
                bg_fill: self.base,
                weak_bg_fill: self.base_weak,
                bg_stroke: egui::Stroke::new(0.0, egui::Color32::TRANSPARENT),
                fg_stroke: egui::Stroke::new(1.0, self.text_weak),
                corner_radius: egui::CornerRadius::same(4),
                expansion: 0.0,
            };
            visuals.widgets.inactive = egui::style::WidgetVisuals {
                bg_fill: self.base_alt,
                weak_bg_fill: self.base_strong,
                bg_stroke: egui::Stroke::new(0.0, egui::Color32::TRANSPARENT),
                fg_stroke: egui::Stroke::new(1.0, self.text),
                corner_radius: egui::CornerRadius::same(4),
                expansion: 0.0,
            };
            visuals.widgets.hovered = egui::style::WidgetVisuals {
                bg_fill: accent_bg,
                weak_bg_fill: accent_bg_weak,
                bg_stroke: egui::Stroke::new(1.0, self.accent),
                fg_stroke: egui::Stroke::new(1.0, self.accent),
                corner_radius: egui::CornerRadius::same(4),
                expansion: 0.0,
            };
            visuals.widgets.active = egui::style::WidgetVisuals {
                bg_fill: self.base_alt,
                weak_bg_fill: self.base_strong,
                bg_stroke: egui::Stroke::new(0.0, self.text_strong),
                fg_stroke: egui::Stroke::new(1.0, self.text_weak),
                corner_radius: egui::CornerRadius::same(4),
                expansion: 0.0,
            };
            visuals.widgets.open = visuals.widgets.active.clone();

            visuals.window_shadow = egui::Shadow {
                offset: [0, 5],
                blur: 10,
                spread: 0,
                color: egui::Color32::from_black_alpha(80),
            };
            visuals.popup_shadow = visuals.window_shadow.clone();

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
        ctx.style_mut(|style| {
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
