use crate::{prelude::*, Direction};
use gpui::{px, AnyElement, Pixels};

#[derive(IntoElement)]
pub struct Card {
    base: Div,
    direction: Direction,
    padding: Pixels,
    margin: Pixels,
    radius: Pixels,
    border_width: Pixels,
    border_color: Option<Hsla>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            base: div(),
            direction: Direction::Vertical,
            padding: px(16.0),
            margin: px(16.0),
            radius: px(4.0),
            border_width: px(1.0),
            border_color: None,
        }
    }

    /// Set the direction of the Radio group. Default is `Direction::Horizontal`.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// This is a convenience method for setting the direction to horizontal without passing an argument.
    pub fn direction_horizontal(mut self) -> Self {
        self.direction = Direction::Horizontal;
        self
    }

    pub fn padding(mut self, v: f32) -> Self {
        self.padding = px(v);
        self
    }

    pub fn margin(mut self, v: f32) -> Self {
        self.margin = px(v);
        self
    }

    pub fn radius(mut self, v: f32) -> Self {
        self.radius = px(v);
        self
    }

    pub fn border_width(mut self, v: f32) -> Self {
        self.border_width = px(v);
        self
    }

    pub fn border_color(mut self, color: impl Into<Hsla>) -> Self {
        self.border_color = Some(color.into());
        self
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.border_color.unwrap_or_else(|| cx.theme().colors.text);

        self.base
            .flex()
            .map(|this| {
                if self.direction.is_vertical() {
                    this.flex_col()
                } else {
                    this.flex_row()
                }
            })
            .flex_col()
            .p(self.padding)
            .m(self.margin)
            .rounded(self.radius)
            .border(self.border_width)
            .border_color(color)
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}
