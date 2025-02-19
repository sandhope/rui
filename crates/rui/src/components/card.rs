use crate::{box_shadow, prelude::*, Direction, Text};
use gpui::{px, AnyElement, FontWeight, Pixels};

#[derive(IntoElement)]
pub struct Card {
    base: Div,
    direction: Direction,
    padding: Edge,
    margin: Edge,
    radius: Pixels,
    border_width: Pixels,
    border_color: Option<Hsla>,
    shadow: bool,
    shadow_hover: bool,
}

impl Card {
    pub fn new() -> Self {
        Self {
            base: div(),
            direction: Direction::Vertical,
            padding: 16.0.into(),
            margin: (16.0, 0.0).into(),
            radius: px(4.0),
            border_width: px(1.0),
            border_color: None,
            shadow: false,
            shadow_hover: false,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.base = self.base.child(
            Text::new(title)
                .margin((-10., 0., 10., 0.))
                .font_weight(FontWeight::SEMIBOLD),
        );
        self
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

    pub fn padding(mut self, v: impl Into<Edge>) -> Self {
        self.padding = v.into();
        self
    }

    pub fn margin(mut self, v: impl Into<Edge>) -> Self {
        self.margin = v.into();
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
        let color = self
            .border_color
            .unwrap_or_else(|| cx.theme().colors.border_variant);

        self.base
            .flex()
            .map(|this| {
                if self.direction.is_vertical() {
                    this.flex_col()
                } else {
                    this.flex_row()
                }
            })
            .padding(self.padding)
            .margin(self.margin)
            .rounded(self.radius)
            .border(self.border_width)
            .border_color(color)
            .when(self.shadow, |this| {
                // this.shadow_sm()
                this.shadow(smallvec::smallvec![box_shadow(
                    0.,
                    4.,
                    20.,
                    0.,
                    crate::rgba(0, 0, 0, 0.1)
                )])
            })
            .when(self.shadow_hover, |this| {
                this.hover(|this| {
                    // this.shadow_md()
                    this.shadow(smallvec::smallvec![box_shadow(
                        0.,
                        8.,
                        30.,
                        0.,
                        crate::rgba(0, 0, 0, 0.2)
                    )])
                })
            })
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}
