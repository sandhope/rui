use crate::{prelude::*, Button, ButtonVariant, Color, Direction};
use gpui::{Corners, Edges};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct ButtonGroup {
    base: Div,
    size: Size,
    children: Vec<Button>,
    direction: Direction,
    disabled: bool,
    variant: ButtonVariant,
    color: Color,
    on_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_row().items_center(),
            size: Size::default(),
            children: Vec::new(),
            direction: Direction::Horizontal,
            disabled: false,
            variant: ButtonVariant::Solid,
            color: Color::Default,
            on_click: None,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.direction = Direction::Vertical;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn child(mut self, child: impl Into<Button>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Button>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn solid(mut self) -> Self {
        self.variant = ButtonVariant::Solid;
        self
    }
    pub fn soft(mut self) -> Self {
        self.variant = ButtonVariant::Soft;
        self
    }
    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }
    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }
    pub fn plain(mut self) -> Self {
        self.variant = ButtonVariant::Plain;
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }
    pub fn primary(mut self) -> Self {
        self.color = Color::Primary;
        self
    }
    pub fn secondary(mut self) -> Self {
        self.color = Color::Secondary;
        self
    }
    pub fn success(mut self) -> Self {
        self.color = Color::Success;
        self
    }
    pub fn warning(mut self) -> Self {
        self.color = Color::Warning;
        self
    }
    pub fn danger(mut self) -> Self {
        self.color = Color::Danger;
        self
    }
}

impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let children_len = self.children.len();

        let border_edges_single = Edges {
            left: true,
            top: true,
            right: true,
            bottom: true,
        };

        let border_edges_last = Edges {
            left: false,
            top: true,
            right: true,
            bottom: true,
        };

        let border_edges_middle = Edges {
            left: false,
            top: true,
            right: true,
            bottom: true,
        };

        let border_corners_first = Corners {
            top_left: true,
            top_right: false,
            bottom_left: true,
            bottom_right: false,
        };

        let border_corners_last = Corners {
            top_left: false,
            top_right: true,
            bottom_left: false,
            bottom_right: true,
        };

        let children = self
            .children
            .into_iter()
            .enumerate()
            .map(|(index, button)| {
                let button = match (children_len, index) {
                    (1, _) => button,
                    (_, 0) => button
                        .border_corners(border_corners_first)
                        .border_edges(border_edges_single),
                    (_, last_index) if last_index == children_len - 1 => button
                        .border_corners(border_corners_last)
                        .border_edges(border_edges_last),
                    _ => button
                        .border_corners(Corners::all(false))
                        .border_edges(border_edges_middle),
                };
                button
                    .disabled(self.disabled)
                    .size(self.size)
                    .variant(self.variant)
                    .color(self.color)
                    .when_some(self.on_click.clone(), |this, on_click| {
                        this.on_click(move |_, window, cx| {
                            on_click(&index, window, cx);
                        })
                    })
            });

        self.base
            .map(|this| match self.direction {
                Direction::Vertical => this.v_flex(),
                Direction::Horizontal => this.h_flex().flex_wrap(),
            })
            .children(children)
    }
}
