// Adapted from https://github.com/catppuccin/egui and
// https://github.com/catppuccin/catppuccin#-palette

use super::{AppTheme, ColorScheme};

const LATTE_COLOR_SCHEME: ColorScheme = ColorScheme {
    // Text
    text: egui::Color32::from_rgb(76, 79, 105),
    // Teal
    text_strong: egui::Color32::from_rgb(23, 146, 153),
    // Subtext 0
    text_weak: egui::Color32::from_rgb(108, 111, 133),
    // Base
    base: egui::Color32::from_rgb(239, 241, 245),
    // Surface 0
    base_alt: egui::Color32::from_rgb(204, 208, 218),
    // Mantle
    base_weak: egui::Color32::from_rgb(230, 233, 239),
    // Crust
    base_strong: egui::Color32::from_rgb(220, 224, 232),
    // Rosewater
    accent: egui::Color32::from_rgb(220, 138, 120),
    // Blue
    link: egui::Color32::from_rgb(30, 102, 245),
    // Yellow
    warn: egui::Color32::from_rgb(223, 142, 29),
    // Red
    error: egui::Color32::from_rgb(210, 15, 57),
    dark: false,
};

const MOCHA_COLOR_SCHEME: ColorScheme = ColorScheme {
    // Text
    text: egui::Color32::from_rgb(205, 214, 244),
    // Teal
    text_strong: egui::Color32::from_rgb(148, 226, 213),
    // Subtext 0
    text_weak: egui::Color32::from_rgb(166, 173, 200),
    // Base
    base: egui::Color32::from_rgb(30, 30, 46),
    // Surface 0
    base_alt: egui::Color32::from_rgb(49, 50, 68),
    // Mantle
    base_weak: egui::Color32::from_rgb(24, 24, 37),
    // Crust
    base_strong: egui::Color32::from_rgb(17, 17, 27),
    // Rosewater
    accent: egui::Color32::from_rgb(245, 224, 220),
    // Blue
    link: egui::Color32::from_rgb(137, 180, 250),
    // Yellow
    warn: egui::Color32::from_rgb(249, 226, 175),
    // Red
    error: egui::Color32::from_rgb(243, 139, 168),
    dark: true,
};

pub struct Latte;

impl AppTheme for Latte {
    fn color_scheme(&self) -> &ColorScheme {
        &LATTE_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "[Catppuccin] Latte"
    }
}

pub struct Mocha;

impl AppTheme for Mocha {
    fn color_scheme(&self) -> &ColorScheme {
        &MOCHA_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "[Catppuccin] Mocha"
    }
}
