use crate::{prelude::*, Direction, Text};
use gpui::px;

/// A divider that can be either vertical or horizontal.
#[derive(IntoElement)]
pub struct Divider {
    base: Div,
    text: Option<Text>,
    direction: Direction,
    color: Option<Hsla>,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            base: div().w_full(),
            direction: Direction::Horizontal,
            text: None,
            color: None,
        }
    }

    pub fn vertical() -> Self {
        Self {
            base: div().h_full(),
            direction: Direction::Vertical,
            text: None,
            color: None,
        }
    }

    pub fn text(mut self, text: impl Into<Text>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Styled for Divider {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Divider {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        self.base
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .child(
                div()
                    .absolute()
                    .map(|this| match self.direction {
                        Direction::Vertical => this.w(px(1.)).h_full(),
                        Direction::Horizontal => this.h(px(1.)).w_full(),
                    })
                    .bg(self.color.unwrap_or(cx.theme().colors.border_variant)),
            )
            .when_some(self.text, |this, text| {
                this.child(
                    div()
                        .px_2()
                        .py_1()
                        .mx_auto()
                        .text_xs()
                        .bg(cx.theme().colors.bg)
                        .child(text),
                )
            })
    }
}
