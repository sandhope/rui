use gpui::Hsla;

use crate::hsl;

#[derive(Debug, Clone, Copy, Default)]
pub struct ThemeColor {
    pub primary: Hsla,
    /// Border color. Used for most borders, is usually a high contrast color.
    pub border: Hsla,
    /// Fill Color. Used for the accent fill color of an icon.
    ///
    /// This might be used to show when a toggleable icon button is selected.
    pub icon_accent: Hsla,
    /// Background Color. Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub element_hover: Hsla,
}

/// The default colors for the theme.
impl ThemeColor {
    pub fn light() -> Self {
        Self {
            primary: hsl(216., 100., 49.),
            border: hsl(210., 1., 67.),
            icon_accent: hsl(216., 100., 49.),
            element_hover: hsl(216., 100., 49.),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: hsl(210., 100., 66.),
            border: hsl(0., 0., 98.),
            icon_accent: hsl(210., 100., 66.),
            element_hover: hsl(210., 100., 66.),
        }
    }
}
