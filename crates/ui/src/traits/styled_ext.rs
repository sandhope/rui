use gpui::Styled;

pub enum Padding {
    Top,
    Bottom,
    Left,
    Right,
    Horizontal,
    Vertical,
    All,
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

    fn padding(
        mut self,
        direction: Padding,
        length: impl std::clone::Clone + Into<gpui::DefiniteLength>,
    ) -> Self {
        let style = self.style();
        match direction {
            Padding::Top => style.padding.top = Some(length.into()),
            Padding::Bottom => style.padding.bottom = Some(length.into()),
            Padding::Left => style.padding.left = Some(length.into()),
            Padding::Right => style.padding.right = Some(length.into()),
            Padding::Horizontal => {
                style.padding.left = Some(length.clone().into());
                style.padding.right = Some(length.into());
            }
            Padding::Vertical => {
                style.padding.top = Some(length.clone().into());
                style.padding.bottom = Some(length.into());
            }
            Padding::All => {
                style.padding.top = Some(length.clone().into());
                style.padding.bottom = Some(length.clone().into());
                style.padding.left = Some(length.clone().into());
                style.padding.right = Some(length.into());
            }
        }
        self
    }
}

impl<E: Styled> StyledExt for E {}
