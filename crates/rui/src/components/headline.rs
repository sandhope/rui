use crate::prelude::*;
use gpui::Rems;

/// The size of a [`Headline`] element
///
/// Defaults to a Major Second scale.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum HeadlineSize {
    /// An extra small headline - `~14px` @16px/rem
    XSmall,
    /// A small headline - `16px` @16px/rem
    Small,
    #[default]
    /// A medium headline - `~18px` @16px/rem
    Medium,
    /// A large headline - `~20px` @16px/rem
    Large,
    /// An extra large headline - `~22px` @16px/rem
    XLarge,
}

impl HeadlineSize {
    /// Returns the headline size in rems.
    pub fn rems(self) -> Rems {
        match self {
            Self::XSmall => rems(0.88),
            Self::Small => rems(1.0),
            Self::Medium => rems(1.125),
            Self::Large => rems(1.27),
            Self::XLarge => rems(1.43),
        }
    }

    /// Returns the line height for the headline size.
    pub fn line_height(self) -> Rems {
        match self {
            Self::XSmall => rems(1.6),
            Self::Small => rems(1.6),
            Self::Medium => rems(1.6),
            Self::Large => rems(1.6),
            Self::XLarge => rems(1.6),
        }
    }
}

/// A headline element, used to emphasize some text and
/// create a visual hierarchy.
#[derive(IntoElement)]
pub struct Headline {
    base: Div,
    size: HeadlineSize,
    text: SharedString,
    color: Option<Hsla>,
}

impl Headline {
    /// Create a new headline element.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            size: HeadlineSize::default(),
            text: text.into(),
            color: None,
        }
    }

    /// Set the size of the headline.
    pub fn size(mut self, size: HeadlineSize) -> Self {
        self.size = size;
        self
    }

    /// Set the color of the headline.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl RenderOnce for Headline {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base
            .line_height(self.size.line_height())
            .text_size(self.size.rems())
            .text_color(self.color.unwrap_or(window.text_style().color))
            .child(self.text)
    }
}

impl Styled for Headline {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}
