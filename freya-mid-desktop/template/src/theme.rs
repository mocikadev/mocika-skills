#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
    Auto,
}

impl ThemeMode {
    pub fn next(self) -> Self {
        match self {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Auto,
            ThemeMode::Auto => ThemeMode::Dark,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
            ThemeMode::Auto => "Auto",
        }
    }

    pub fn resolved(self) -> Self {
        match self {
            ThemeMode::Auto => ThemeMode::Dark,
            _ => self,
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub struct ThemeTokens {
    pub bg_nav:      (u8, u8, u8),
    pub bg_stage:    (u8, u8, u8),
    pub bg_card:     (u8, u8, u8),
    pub bg_elevated: (u8, u8, u8),
    pub border_subtle: (u8, u8, u8, u8),
    pub border:      (u8, u8, u8, u8),
    pub text_primary:   (u8, u8, u8),
    pub text_secondary: (u8, u8, u8),
    pub text_muted:     (u8, u8, u8),
    pub text_disabled:  (u8, u8, u8),
}

pub fn theme_tokens(mode: ThemeMode) -> ThemeTokens {
    match mode.resolved() {
        ThemeMode::Dark => ThemeTokens {
            bg_nav:      (17,  17,  20),
            bg_stage:    (22,  22,  26),
            bg_card:     (28,  28,  34),
            bg_elevated: (38,  38,  46),
            border_subtle: (255, 255, 255, 15),
            border:        (255, 255, 255, 25),
            text_primary:   (228, 228, 231),
            text_secondary: (161, 161, 170),
            text_muted:     (113, 113, 122),
            text_disabled:  (82,  82,  91),
        },
        ThemeMode::Light => ThemeTokens {
            bg_nav:      (228, 228, 231),
            bg_stage:    (250, 250, 252),
            bg_card:     (255, 255, 255),
            bg_elevated: (240, 240, 243),
            border_subtle: (0, 0, 0, 10),
            border:        (0, 0, 0, 20),
            text_primary:   (24,  24,  27),
            text_secondary: (63,  63,  70),
            text_muted:     (113, 113, 122),
            text_disabled:  (161, 161, 170),
        },
        ThemeMode::Auto => unreachable!(),
    }
}

pub const DANGER_RED:  (u8, u8, u8) = (239, 68, 68);
pub const RADIUS_CARD: f32 = 12.0;
pub const RADIUS_CTRL: f32 = 6.0;

pub const fn with_alpha(rgb: (u8, u8, u8), a: u8) -> (u8, u8, u8, u8) {
    (rgb.0, rgb.1, rgb.2, a)
}
