use gpui::{px, Styled};
use ui_macros::box_style_methods;

// top, right, bottom, left
pub struct Edge(f32, f32, f32, f32);

impl From<f32> for Edge {
    fn from(v: f32) -> Self {
        Edge(v, v, v, v)
    }
}

impl From<(f32, f32)> for Edge {
    fn from(v: (f32, f32)) -> Self {
        Edge(v.0, v.1, v.0, v.1)
    }
}

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

    fn margin(self, value: impl Into<Edge>) -> Self {
        let v = value.into();
        self.mt(px(v.0)).mr(px(v.1)).mb(px(v.2)).ml(px(v.3))
    }

    box_style_methods!();
}

impl<E: Styled> StyledExt for E {}
