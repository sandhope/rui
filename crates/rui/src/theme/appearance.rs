use gpui::{Hsla, WindowAppearance};

/// The appearance of the theme.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Appearance {
    /// A light appearance.
    Light,
    /// A dark appearance.
    Dark,
}

impl Appearance {
    /// Returns whether the appearance is light.
    pub fn is_light(&self) -> bool {
        match self {
            Self::Light => true,
            Self::Dark => false,
        }
    }

    /// Converts the Appearance to its string representation.
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    /// Toggles the appearance between Light and Dark.
    pub fn toggle(&mut self) {
        *self = match *self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        };
    }

    pub fn bg_color(&self) -> Hsla {
        match self {
            Self::Light => gpui::white(),
            Self::Dark => gpui::black(),
        }
    }
}

impl From<WindowAppearance> for Appearance {
    fn from(value: WindowAppearance) -> Self {
        match value {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
        }
    }
}
