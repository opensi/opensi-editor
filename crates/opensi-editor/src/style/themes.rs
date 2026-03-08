use super::{AppTheme, ColorScheme};

pub struct CatppuccinLatte;

impl AppTheme for CatppuccinLatte {
    fn color_scheme(&self) -> &ColorScheme {
        &LATTE_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Catppuccin Latte"
    }
}

pub struct CatppuccinMocha;

impl AppTheme for CatppuccinMocha {
    fn color_scheme(&self) -> &ColorScheme {
        &MOCHA_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Catppuccin Mocha"
    }
}

pub struct Dracula;

impl AppTheme for Dracula {
    fn color_scheme(&self) -> &ColorScheme {
        &DRACULA_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Dracula"
    }
}

pub struct Nord;

impl AppTheme for Nord {
    fn color_scheme(&self) -> &ColorScheme {
        &NORD_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Nord"
    }
}

pub struct Gruvbox;

impl AppTheme for Gruvbox {
    fn color_scheme(&self) -> &ColorScheme {
        &GRUVBOX_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "Gruvbox"
    }
}

pub struct OneDark;

impl AppTheme for OneDark {
    fn color_scheme(&self) -> &ColorScheme {
        &ONE_DARK_COLOR_SCHEME
    }

    fn name(&self) -> &'static str {
        "One Dark"
    }
}

const DRACULA_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(255, 255, 255),
    text_strong: egui::Color32::from_rgb(189, 147, 249),
    text_weak: egui::Color32::from_rgb(139, 233, 253),
    base: egui::Color32::from_rgb(40, 42, 54),
    base_weak: egui::Color32::from_rgb(68, 71, 90),
    base_alt: egui::Color32::from_rgb(68, 71, 90),
    base_strong: egui::Color32::from_rgb(24, 24, 37),
    accent: egui::Color32::from_rgb(255, 121, 198),
    link: egui::Color32::from_rgb(139, 233, 253),
    warn: egui::Color32::from_rgb(241, 250, 140),
    error: egui::Color32::from_rgb(255, 85, 85),
    dark: true,
};

const NORD_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(216, 222, 233),
    text_strong: egui::Color32::from_rgb(136, 192, 208),
    text_weak: egui::Color32::from_rgb(129, 161, 193),
    base: egui::Color32::from_rgb(46, 52, 64),
    base_weak: egui::Color32::from_rgb(59, 66, 82),
    base_alt: egui::Color32::from_rgb(67, 76, 94),
    base_strong: egui::Color32::from_rgb(35, 38, 48),
    accent: egui::Color32::from_rgb(136, 192, 208),
    link: egui::Color32::from_rgb(94, 129, 172),
    warn: egui::Color32::from_rgb(235, 203, 139),
    error: egui::Color32::from_rgb(191, 97, 106),
    dark: true,
};

const GRUVBOX_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(235, 219, 178),
    text_strong: egui::Color32::from_rgb(251, 241, 199),
    text_weak: egui::Color32::from_rgb(168, 153, 132),
    base: egui::Color32::from_rgb(40, 40, 40),
    base_weak: egui::Color32::from_rgb(60, 56, 54),
    base_alt: egui::Color32::from_rgb(80, 73, 69),
    base_strong: egui::Color32::from_rgb(30, 30, 30),
    accent: egui::Color32::from_rgb(184, 187, 38),
    link: egui::Color32::from_rgb(131, 165, 152),
    warn: egui::Color32::from_rgb(250, 189, 47),
    error: egui::Color32::from_rgb(204, 36, 29),
    dark: true,
};

const ONE_DARK_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(171, 178, 191),
    text_strong: egui::Color32::from_rgb(197, 206, 224),
    text_weak: egui::Color32::from_rgb(92, 99, 112),
    base: egui::Color32::from_rgb(40, 44, 52),
    base_weak: egui::Color32::from_rgb(48, 54, 66),
    base_alt: egui::Color32::from_rgb(56, 62, 74),
    base_strong: egui::Color32::from_rgb(30, 34, 40),
    accent: egui::Color32::from_rgb(97, 175, 239),
    link: egui::Color32::from_rgb(198, 120, 221),
    warn: egui::Color32::from_rgb(224, 175, 104),
    error: egui::Color32::from_rgb(224, 108, 117),
    dark: true,
};

const LATTE_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(76, 79, 105),
    text_strong: egui::Color32::from_rgb(23, 146, 153),
    text_weak: egui::Color32::from_rgb(108, 111, 133),
    base: egui::Color32::from_rgb(239, 241, 245),
    base_alt: egui::Color32::from_rgb(204, 208, 218),
    base_weak: egui::Color32::from_rgb(230, 233, 239),
    base_strong: egui::Color32::from_rgb(220, 224, 232),
    accent: egui::Color32::from_rgb(220, 138, 120),
    link: egui::Color32::from_rgb(30, 102, 245),
    warn: egui::Color32::from_rgb(223, 142, 29),
    error: egui::Color32::from_rgb(210, 15, 57),
    dark: false,
};

const MOCHA_COLOR_SCHEME: ColorScheme = ColorScheme {
    text: egui::Color32::from_rgb(205, 214, 244),
    text_strong: egui::Color32::from_rgb(148, 226, 213),
    text_weak: egui::Color32::from_rgb(166, 173, 200),
    base: egui::Color32::from_rgb(30, 30, 46),
    base_alt: egui::Color32::from_rgb(49, 50, 68),
    base_weak: egui::Color32::from_rgb(24, 24, 37),
    base_strong: egui::Color32::from_rgb(17, 17, 27),
    accent: egui::Color32::from_rgb(245, 224, 220),
    link: egui::Color32::from_rgb(137, 180, 250),
    warn: egui::Color32::from_rgb(249, 226, 175),
    error: egui::Color32::from_rgb(243, 139, 168),
    dark: true,
};
