use gpui::{px, Styled};
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
}

impl<E: Styled> StyledExt for E {}
