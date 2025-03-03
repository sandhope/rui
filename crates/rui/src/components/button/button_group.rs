use crate::{prelude::*, Button};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct ButtonGroup {
    base: Div,
    children: Vec<Button>,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_row().items_center(),
            children: Vec::new(),
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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
}

impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(
            self.children
                .into_iter()
                .enumerate()
                .map(|(index, button)| {
                    button
                        .disabled(self.disabled)
                        .bg(gpui::black().opacity(0.5))
                        .when_some(self.on_click.clone(), |this, on_click| {
                            this.on_click(move |_, window, cx| {
                                on_click(&index, window, cx);
                            })
                        })
                }),
        )
    }
}
