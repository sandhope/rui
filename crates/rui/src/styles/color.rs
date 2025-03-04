use crate::ActiveTheme;
use gpui::{App, Hsla};

/// Sets a color that has a consistent meaning across all themes.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    Default,
    Primary,
    Secondary,
    Info,
    Success,
    Warning,
    Danger,
    Custom(Hsla),
}

impl Color {
    /// Returns the Color's HSLA value.
    pub fn color(&self, cx: &App) -> Hsla {
        match self {
            Color::Default => cx.theme().colors.text,
            Color::Primary => cx.theme().colors.primary,
            Color::Secondary => cx.theme().colors.secondary,
            Color::Info => cx.theme().colors.info,
            Color::Success => cx.theme().colors.success,
            Color::Warning => cx.theme().colors.warning,
            Color::Danger => cx.theme().colors.danger,
            Color::Custom(color) => *color,
        }
    }

    const fn hsl(h: f32, s: f32, l: f32) -> Hsla {
        Hsla {
            h: h / 360.0,
            s: s / 100.0,
            l: l / 100.0,
            a: 1.0,
        }
    }

    pub const fn black() -> Hsla {
        Self::hsl(0., 0., 0.)
    }
    pub const fn white() -> Hsla {
        Self::hsl(0., 0., 100.)
    }
    pub const fn red() -> Hsla {
        Self::hsl(0., 72., 51.)
    }
    pub const fn green() -> Hsla {
        Self::hsl(142., 76., 36.)
    }
    pub const fn gray() -> Hsla {
        Self::hsl(0., 0., 50.)
    }
    pub const fn blue() -> Hsla {
        Self::hsl(221., 83., 53.)
    }
    pub const fn teal() -> Hsla {
        Self::hsl(175., 84., 32.)
    }
    pub const fn pink() -> Hsla {
        Self::hsl(333., 71., 51.)
    }
    pub const fn purple() -> Hsla {
        Self::hsl(271., 81., 56.)
    }
    pub const fn cyan() -> Hsla {
        Self::hsl(192., 91., 36.)
    }
    pub const fn orange() -> Hsla {
        Self::hsl(21., 90., 48.)
    }
    pub const fn yellow() -> Hsla {
        Self::hsl(50., 98., 64.)
    }
    pub const fn transparent() -> Hsla {
        gpui::transparent_black()
    }
}

impl From<Hsla> for Color {
    fn from(color: Hsla) -> Self {
        Color::Custom(color)
    }
}
