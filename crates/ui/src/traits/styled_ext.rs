use gpui::{px, Styled};

pub enum Padding {
    Top(f32),
    Bottom(f32),
    Left(f32),
    Right(f32),
    Horizontal(f32),
    Vertical(f32),
    All(f32, f32, f32, f32),
}

impl From<f32> for Padding {
    fn from(v: f32) -> Self {
        Padding::All(v, v, v, v)
    }
}

impl From<(f32, f32)> for Padding {
    fn from(v: (f32, f32)) -> Self {
        Padding::All(v.0, v.1, v.0, v.1)
    }
}

impl From<(f32, f32, f32, f32)> for Padding {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Padding::All(value.0, value.1, value.2, value.3)
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

    fn padding(mut self, value: impl Into<Padding>) -> Self {
        let style = self.style();
        match value.into() {
            Padding::Top(v) => style.padding.top = Some(px(v).into()),
            Padding::Bottom(v) => style.padding.bottom = Some(px(v).into()),
            Padding::Left(v) => style.padding.left = Some(px(v).into()),
            Padding::Right(v) => style.padding.right = Some(px(v).into()),
            Padding::Horizontal(v) => {
                style.padding.left = Some(px(v).into());
                style.padding.right = Some(px(v).into());
            }
            Padding::Vertical(v) => {
                style.padding.top = Some(px(v).into());
                style.padding.bottom = Some(px(v).into());
            }
            Padding::All(top, right, bottom, left) => {
                style.padding.top = Some(px(top).into());
                style.padding.right = Some(px(right).into());
                style.padding.bottom = Some(px(bottom).into());
                style.padding.left = Some(px(left).into());
            }
        }
        self
    }
}

impl<E: Styled> StyledExt for E {}
