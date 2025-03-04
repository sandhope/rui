use gpui::Hsla;

use crate::hsl;

use super::Appearance;

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub default: Hsla,
    pub primary: Hsla,
    pub secondary: Hsla,
    pub info: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,
    pub primary_disabled: Hsla,
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
    pub element_bg: Hsla,
    /// Background Color. Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub element_hover: Hsla,

    pub text: Hsla,
    pub bg: Hsla,
    pub fg: Hsla,
    pub switch_checked_bg: Hsla,
    pub switch_checked_hover_bg: Hsla,
    pub switch_unchecked_bg: Hsla,
    pub switch_unchecked_hover_bg: Hsla,
}

/// The default colors for the theme.
impl ThemeColors {
    pub fn light() -> Self {
        Self {
            default: hsl(240., 6., 10.),
            primary: hsl(212., 100., 47.),
            secondary: hsl(270., 67., 47.),
            info: hsl(240., 5., 84.),
            success: hsl(146., 79., 44.),
            warning: hsl(37., 91., 55.),
            danger: hsl(339., 90., 51.),
            primary_disabled: hsl(208., 97., 85.),
            border: hsl(210., 1., 67.),
            border_variant: hsl(240.0, 5.9, 90.0),
            icon_accent: hsl(216., 100., 49.),
            element_bg: hsl(240., 2., 92.),
            element_hover: hsl(216., 100., 49.),
            text: hsl(214., 11., 12.),
            bg: hsl(0., 0., 100.),
            fg: hsl(0., 0., 0.),
            switch_checked_bg: hsl(126., 50., 47.),
            switch_checked_hover_bg: hsl(127., 51., 38.),
            switch_unchecked_bg: hsl(240., 5., 85.),
            switch_unchecked_hover_bg: hsl(240., 10., 75.),
        }
    }

    pub fn dark() -> Self {
        Self {
            default: hsl(0., 0., 90.),
            primary: hsl(210.0, 100., 66.),
            secondary: hsl(270., 67., 47.),
            info: hsl(240., 5., 84.),
            success: hsl(146., 79., 44.),
            warning: hsl(37., 91., 55.),
            danger: hsl(339., 90., 51.),
            primary_disabled: hsl(214., 73., 34.),
            border: hsl(0., 0., 98.),
            border_variant: hsl(240.0, 3.7, 16.9),
            icon_accent: hsl(210., 100., 66.),
            element_bg: hsl(0., 0., 100.),
            element_hover: hsl(210., 100., 66.),
            text: hsl(0., 0., 98.),
            bg: hsl(0., 0., 0.),
            fg: hsl(0., 0., 100.),
            switch_checked_bg: hsl(126., 50., 47.),
            switch_checked_hover_bg: hsl(127., 51., 38.),
            switch_unchecked_bg: hsl(240., 5., 85.),
            switch_unchecked_hover_bg: hsl(240., 10., 75.),
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
