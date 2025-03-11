use crate::{prelude::*, Direction, Text};
use gpui::{px, AbsoluteLength};

/// A divider that can be either vertical or horizontal.
#[derive(IntoElement)]
pub struct Divider {
    base: Div,
    style: DividerStyle,
    direction: Direction,
    color: Option<Hsla>,
    text: Option<Text>,
}

#[derive(Clone, Copy, PartialEq)]
enum DividerStyle {
    Solid,
    Dashed,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            base: div().w_full(),
            style: DividerStyle::Solid,
            direction: Direction::Horizontal,
            color: None,
            text: None,
        }
    }

    pub fn vertical() -> Self {
        Self {
            base: div().h_full(),
            style: DividerStyle::Solid,
            direction: Direction::Vertical,
            color: None,
            text: None,
        }
    }

    pub fn dashed(mut self) -> Self {
        self.style = DividerStyle::Dashed;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn text(mut self, text: impl Into<Text>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl Styled for Divider {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Divider {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.style {
            DividerStyle::Solid => self.render_solid(cx).into_any_element(),
            DividerStyle::Dashed => self.render_dashed(cx).into_any_element(),
        }
    }
}

impl Divider {
    pub fn render_solid(self, cx: &mut App) -> impl IntoElement {
        self.base
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .child(
                div()
                    .absolute()
                    .map(|this| match self.direction {
                        Direction::Horizontal => this.h_px().w_full(),
                        Direction::Vertical => this.w_px().h_full(),
                    })
                    .bg(self.color.unwrap_or(cx.theme().colors.border)),
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

    // TODO: Use canvas or a shader here
    // This obviously is a short term approach
    pub fn render_dashed(self, cx: &mut App) -> impl IntoElement {
        let segment_count = 128;
        let segment_min_w = px(6.);
        let line = match self.direction {
            Direction::Horizontal => h_flex().w_full().h_px(),
            Direction::Vertical => v_flex().w_px().h_full(),
        };
        let (w, h) = match self.direction {
            Direction::Horizontal => (segment_min_w, px(1.)),
            Direction::Vertical => (px(1.), segment_min_w),
        };
        let color = self.color.unwrap_or(cx.theme().colors.border);

        self.base
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .min_w(px(3.))
            .min_h(px(3.))
            .child(line.absolute().gap(segment_min_w).children(
                (0..segment_count).map(|_| div().flex_grow().flex_shrink_0().w(w).h(h).bg(color)),
            ))
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

#[derive(IntoElement)]
pub struct DividerTitle {
    base: Div,
    color: Option<Hsla>,
    text: Text,
    text_size: AbsoluteLength,
}

impl DividerTitle {
    pub fn new(text: impl Into<Text>) -> Self {
        Self {
            base: div().w_full(),
            color: None,
            text: text.into(),
            text_size: px(10.).into(),
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn text_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.text_size = size.into();
        self
    }
}

impl Styled for DividerTitle {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for DividerTitle {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(cx.theme().colors.border);

        self.base
            .flex()
            .items_center()
            .gap_3()
            .pb_1()
            .child(div().h_px().w_4().bg(color))
            .child(div().flex_none().text_size(self.text_size).child(self.text))
            .child(div().h_px().w_full().flex_1().bg(color))
    }
}
