use super::{AppTheme, ColorScheme};

pub struct OpenSIBlue;

impl AppTheme for OpenSIBlue {
    fn colorscheme_day(&self) -> &ColorScheme {
        &BLUE_DAY_COLOR_SCHEME
    }

    fn colorscheme_night(&self) -> &ColorScheme {
        &BLUE_NIGHT_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Big Brain Blue"
    }
}

pub struct OpenSIGreen;

impl AppTheme for OpenSIGreen {
    fn colorscheme_day(&self) -> &ColorScheme {
        &GREEN_DAY_COLOR_SCHEME
    }

    fn colorscheme_night(&self) -> &ColorScheme {
        &GREEN_NIGHT_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Grand Genius Green"
    }
}

pub struct OpenSIOrange;

impl AppTheme for OpenSIOrange {
    fn colorscheme_day(&self) -> &ColorScheme {
        &ORANGE_DAY_COLOR_SCHEME
    }

    fn colorscheme_night(&self) -> &ColorScheme {
        &ORANGE_NIGHT_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Outsmarted & Outplayed Orange"
    }
}

pub struct OpenSIPeach;

impl AppTheme for OpenSIPeach {
    fn colorscheme_day(&self) -> &ColorScheme {
        &PEACH_DAY_COLOR_SCHEME
    }

    fn colorscheme_night(&self) -> &ColorScheme {
        &PEACH_NIGHT_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Puzzle Piece Peach"
    }
}

const BLUE_NIGHT_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(230, 233, 239),
    text_strong: egui::Color32::from_rgb(240, 243, 248),
    text_weak: egui::Color32::from_rgb(141, 148, 161),
    base: egui::Color32::from_rgb(25, 28, 33),
    base_weak: egui::Color32::from_rgb(24, 27, 33),
    base_alt: egui::Color32::from_rgb(30, 34, 42),
    base_strong: egui::Color32::from_rgb(20, 22, 26),
    accent: egui::Color32::from_rgb(111, 157, 255),
    link: egui::Color32::from_rgb(111, 157, 255),
    warn: egui::Color32::from_rgb(224, 175, 104),
    error: egui::Color32::from_rgb(224, 108, 117),
    dark: true,
};

const GREEN_NIGHT_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(230, 239, 233),
    text_strong: egui::Color32::from_rgb(240, 248, 243),
    text_weak: egui::Color32::from_rgb(141, 161, 148),
    base: egui::Color32::from_rgb(25, 33, 28),
    base_weak: egui::Color32::from_rgb(24, 33, 27),
    base_alt: egui::Color32::from_rgb(30, 42, 34),
    base_strong: egui::Color32::from_rgb(20, 26, 22),
    accent: egui::Color32::from_rgb(111, 214, 143),
    link: egui::Color32::from_rgb(111, 214, 143),
    warn: egui::Color32::from_rgb(224, 175, 104),
    error: egui::Color32::from_rgb(224, 108, 117),
    dark: true,
};

const ORANGE_NIGHT_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(239, 235, 230),
    text_strong: egui::Color32::from_rgb(248, 245, 240),
    text_weak: egui::Color32::from_rgb(161, 152, 141),
    base: egui::Color32::from_rgb(33, 30, 25),
    base_weak: egui::Color32::from_rgb(33, 29, 24),
    base_alt: egui::Color32::from_rgb(42, 37, 30),
    base_strong: egui::Color32::from_rgb(26, 23, 20),
    accent: egui::Color32::from_rgb(255, 179, 87),
    link: egui::Color32::from_rgb(255, 179, 87),
    warn: egui::Color32::from_rgb(240, 200, 110),
    error: egui::Color32::from_rgb(224, 108, 117),
    dark: true,
};

const PEACH_NIGHT_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(239, 232, 230),
    text_strong: egui::Color32::from_rgb(248, 243, 240),
    text_weak: egui::Color32::from_rgb(161, 147, 143),
    base: egui::Color32::from_rgb(33, 27, 26),
    base_weak: egui::Color32::from_rgb(33, 26, 25),
    base_alt: egui::Color32::from_rgb(43, 34, 32),
    base_strong: egui::Color32::from_rgb(26, 21, 20),
    accent: egui::Color32::from_rgb(255, 180, 162),
    link: egui::Color32::from_rgb(255, 180, 162),
    warn: egui::Color32::from_rgb(240, 200, 110),
    error: egui::Color32::from_rgb(224, 108, 117),
    dark: true,
};

const BLUE_DAY_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(56, 60, 70),
    text_strong: egui::Color32::from_rgb(28, 32, 44),
    text_weak: egui::Color32::from_rgb(118, 124, 138),
    base: egui::Color32::from_rgb(224, 227, 234),
    base_weak: egui::Color32::from_rgb(218, 221, 229),
    base_alt: egui::Color32::from_rgb(233, 236, 242),
    base_strong: egui::Color32::from_rgb(207, 211, 220),
    accent: egui::Color32::from_rgb(52, 110, 226),
    link: egui::Color32::from_rgb(52, 110, 226),
    warn: egui::Color32::from_rgb(176, 118, 24),
    error: egui::Color32::from_rgb(200, 50, 62),
    dark: false,
};

const GREEN_DAY_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(56, 64, 58),
    text_strong: egui::Color32::from_rgb(28, 40, 32),
    text_weak: egui::Color32::from_rgb(116, 128, 120),
    base: egui::Color32::from_rgb(225, 231, 226),
    base_weak: egui::Color32::from_rgb(219, 226, 220),
    base_alt: egui::Color32::from_rgb(233, 239, 234),
    base_strong: egui::Color32::from_rgb(208, 216, 210),
    accent: egui::Color32::from_rgb(38, 146, 90),
    link: egui::Color32::from_rgb(38, 146, 90),
    warn: egui::Color32::from_rgb(176, 118, 24),
    error: egui::Color32::from_rgb(200, 50, 62),
    dark: false,
};

const ORANGE_DAY_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(68, 62, 55),
    text_strong: egui::Color32::from_rgb(44, 38, 30),
    text_weak: egui::Color32::from_rgb(132, 124, 115),
    base: egui::Color32::from_rgb(233, 229, 223),
    base_weak: egui::Color32::from_rgb(228, 223, 217),
    base_alt: egui::Color32::from_rgb(240, 236, 230),
    base_strong: egui::Color32::from_rgb(217, 211, 203),
    accent: egui::Color32::from_rgb(198, 118, 28),
    link: egui::Color32::from_rgb(198, 118, 28),
    warn: egui::Color32::from_rgb(176, 118, 24),
    error: egui::Color32::from_rgb(200, 50, 62),
    dark: false,
};

const PEACH_DAY_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(70, 60, 57),
    text_strong: egui::Color32::from_rgb(46, 36, 34),
    text_weak: egui::Color32::from_rgb(134, 122, 118),
    base: egui::Color32::from_rgb(234, 227, 225),
    base_weak: egui::Color32::from_rgb(229, 222, 220),
    base_alt: egui::Color32::from_rgb(241, 235, 233),
    base_strong: egui::Color32::from_rgb(218, 210, 207),
    accent: egui::Color32::from_rgb(212, 104, 80),
    link: egui::Color32::from_rgb(212, 104, 80),
    warn: egui::Color32::from_rgb(176, 118, 24),
    error: egui::Color32::from_rgb(200, 50, 62),
    dark: false,
};
