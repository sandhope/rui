use crate::{ActiveTheme, ElevationIndex};
use gpui::{px, App, Styled};
use rui_macros::box_style_methods;

// top, right, bottom, left
pub struct Edge(f32, f32, f32, f32);

/// Converts a single value into an `Edge` with equal top, right, bottom, and left values.
/// Corresponds to CSS shorthand for setting all sides equally.
impl From<f32> for Edge {
    fn from(v: f32) -> Self {
        Edge(v, v, v, v)
    }
}

/// Converts a tuple of two values into an `Edge` with top and bottom equal to the first value,
/// and right and left equal to the second value.
/// Corresponds to CSS shorthand for setting vertical and horizontal values.
impl From<(f32, f32)> for Edge {
    fn from(v: (f32, f32)) -> Self {
        Edge(v.0, v.1, v.0, v.1)
    }
}

/// Converts a tuple of four values into an `Edge` with top, right, bottom, and left values.
/// Corresponds to CSS shorthand for setting each side individually.
impl From<(f32, f32, f32, f32)> for Edge {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Edge(v.0, v.1, v.2, v.3)
    }
}

fn elevated<E: Styled>(this: E, cx: &App, index: ElevationIndex) -> E {
    this.bg(cx.theme().colors.bg_elevated_surface)
        .rounded_lg()
        .border_1()
        .border_color(cx.theme().colors.border_variant)
        .shadow(index.shadow(cx))
}

fn elevated_borderless<E: Styled>(this: E, cx: &mut App, index: ElevationIndex) -> E {
    this.bg(cx.theme().colors.bg_elevated_surface)
        .rounded_lg()
        .shadow(index.shadow(cx))
}

/// Extends [`gpui::Styled`] with Zed-specific styling methods.
pub trait StyledExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    /// Sets the padding of the element, in pixels.
    ///
    /// Converts the input value into an `Edge` and sets padding for top, right, bottom, and left.
    ///
    /// # Examples
    ///
    /// ```
    /// element.padding(10.0); // Sets padding for all sides to 10.0
    /// element.padding((10.0, 20.0)); // Sets vertical padding to 10.0 and horizontal padding to 20.0
    /// element.padding((10.0, 20.0, 30.0, 40.0)); // Sets padding for top, right, bottom, and left respectively
    /// ```
    fn padding(self, value: impl Into<Edge>) -> Self {
        let v = value.into();
        self.pt(px(v.0)).pr(px(v.1)).pb(px(v.2)).pl(px(v.3))
        // let style = self.style();
        // style.padding.top = Some(px(v.0).into());
        // style.padding.right = Some(px(v.1).into());
        // style.padding.bottom = Some(px(v.2).into());
        // style.padding.left = Some(px(v.3).into());
        // self
    }

    /// Sets the margin of the element, in pixels.
    ///
    /// Converts the input value into an `Edge` and sets margin for top, right, bottom, and left.
    ///
    /// # Examples
    ///
    /// ```
    /// element.margin(10.0); // Sets margin for all sides to 10.0
    /// element.margin((10.0, 20.0)); // Sets vertical margin to 10.0 and horizontal margin to 20.0
    /// element.margin((10.0, 20.0, 30.0, 40.0)); // Sets margin for top, right, bottom, and left respectively
    /// ```
    fn margin(self, value: impl Into<Edge>) -> Self {
        let v = value.into();
        self.mt(px(v.0)).mr(px(v.1)).mb(px(v.2)).ml(px(v.3))
    }

    box_style_methods!();

    /// The [`Surface`](ElevationIndex::Surface) elevation level, located above the app background, is the standard level for all elements
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Example Elements: Title Bar, Panel, Tab Bar, Editor
    fn elevation_1(self, cx: &mut App) -> Self {
        elevated(self, cx, ElevationIndex::Surface)
    }

    /// See [`elevation_1`](Self::elevation_1).
    ///
    /// Renders a borderless version [`elevation_1`](Self::elevation_1).
    fn elevation_1_borderless(self, cx: &mut App) -> Self {
        elevated_borderless(self, cx, ElevationIndex::Surface)
    }

    /// Non-Modal Elevated Surfaces appear above the [`Surface`](ElevationIndex::Surface) layer and is used for things that should appear above most UI elements like an editor or panel, but not elements like popovers, context menus, modals, etc.
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Examples: Notifications, Palettes, Detached/Floating Windows, Detached/Floating Panels
    fn elevation_2(self, cx: &App) -> Self {
        elevated(self, cx, ElevationIndex::ElevatedSurface)
    }

    /// See [`elevation_2`](Self::elevation_2).
    ///
    /// Renders a borderless version [`elevation_2`](Self::elevation_2).
    fn elevation_2_borderless(self, cx: &mut App) -> Self {
        elevated_borderless(self, cx, ElevationIndex::ElevatedSurface)
    }

    /// Modal Surfaces are used for elements that should appear above all other UI elements and are located above the wash layer. This is the maximum elevation at which UI elements can be rendered in their default state.
    ///
    /// Elements rendered at this layer should have an enforced behavior: Any interaction outside of the modal will either dismiss the modal or prompt an action (Save your progress, etc) then dismiss the modal.
    ///
    /// If the element does not have this behavior, it should be rendered at the [`Elevated Surface`](ElevationIndex::ElevatedSurface) layer.
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Examples: Settings Modal, Channel Management, Wizards/Setup UI, Dialogs
    fn elevation_3(self, cx: &App) -> Self {
        elevated(self, cx, ElevationIndex::ModalSurface)
    }

    /// See [`elevation_3`](Self::elevation_3).
    ///
    /// Renders a borderless version [`elevation_3`](Self::elevation_3).
    fn elevation_3_borderless(self, cx: &mut App) -> Self {
        elevated_borderless(self, cx, ElevationIndex::ModalSurface)
    }
}

impl<E: Styled> StyledExt for E {}
