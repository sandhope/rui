use gpui::{hsla, Hsla};

use crate::hsl;

use super::Appearance;

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub primary: Hsla,
    /// Border color. Used for most borders, is usually a high contrast color.
    pub border: Hsla,
    /// Border color. Used for deemphasized borders, like a visual divider between two sections
    pub border_variant: Hsla,
    /// Fill Color. Used for the accent fill color of an icon.
    ///
    /// This might be used to show when a toggleable icon button is selected.
    pub icon_accent: Hsla,
    /// Background Color. Used for the background of an element that should have a different background than the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have the same background as the surface it's on, use `ghost_element_background`.
    pub element_background: Hsla,
    /// Background Color. Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub element_hover: Hsla,

    pub text: Hsla,
    pub bg: Hsla,
    pub switch_bg: Hsla,
    pub switch_hover_bg: Hsla,
}

/// The default colors for the theme.
impl ThemeColors {
    pub fn light() -> Self {
        Self {
            primary: hsl(216., 100., 49.),
            border: hsl(210., 1., 67.),
            border_variant: hsl(240.0, 5.9, 90.0),
            icon_accent: hsl(216., 100., 49.),
            element_background: hsl(180., 1., 73.),
            element_hover: hsl(216., 100., 49.),
            text: hsl(214., 11., 12.),
            bg: hsla(0., 0., 1., 1.),
            switch_bg: hsl(126., 50., 47.),
            switch_hover_bg: hsl(127., 51., 39.),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: hsl(210., 100., 66.),
            border: hsl(0., 0., 98.),
            border_variant: hsl(240.0, 3.7, 16.9),
            icon_accent: hsl(210., 100., 66.),
            element_background: hsla(0., 0., 1., 1.),
            element_hover: hsl(210., 100., 66.),
            text: hsl(0., 0., 98.),
            bg: hsla(0., 0., 0., 1.),
            switch_bg: hsl(126., 50., 47.),
            switch_hover_bg: hsl(127., 51., 39.),
        }
    }
}

impl From<Appearance> for ThemeColors {
    fn from(appearance: Appearance) -> Self {
        match appearance {
            Appearance::Light => ThemeColors::light(),
            Appearance::Dark => ThemeColors::dark(),
        }
    }
}
