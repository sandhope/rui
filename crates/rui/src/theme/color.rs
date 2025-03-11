use gpui::Hsla;

use crate::hsl;

use super::Appearance;

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub default: Hsla,
    pub primary: Hsla,
    pub secondary: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,
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
    /// Text Color. Color of muted or deemphasized text. It is a subdued version of the standard text color.
    pub text_muted: Hsla,
    pub bg: Hsla,
    /// Background color. Used for elevated surfaces, like a context menu, popup, or dialog.
    pub bg_elevated_surface: Hsla,
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
            success: hsl(146., 79., 44.),
            warning: hsl(37., 91., 55.),
            danger: hsl(339., 90., 51.),
            border: hsl(210., 1., 67.),
            border_variant: hsl(240., 2., 88.),
            icon_accent: hsl(216., 100., 49.),
            element_bg: hsl(240., 2., 92.),
            element_hover: hsl(216., 100., 49.),
            text: hsl(214., 11., 12.),
            text_muted: hsl(240., 1., 35.),
            bg: hsl(0., 0., 100.),
            bg_elevated_surface: hsl(240., 3., 92.),
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
            success: hsl(146., 79., 44.),
            warning: hsl(37., 91., 55.),
            danger: hsl(339., 90., 51.),
            border: hsl(0., 0., 98.),
            border_variant: hsl(218., 13., 24.),
            icon_accent: hsl(210., 100., 66.),
            element_bg: hsl(0., 0., 100.),
            element_hover: hsl(210., 100., 66.),
            text: hsl(0., 0., 98.),
            text_muted: hsl(221., 12., 70.),
            bg: hsl(0., 0., 0.),
            bg_elevated_surface: hsl(220., 14., 21.),
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
