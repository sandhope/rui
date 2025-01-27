use gpui::{
    div, prelude::*, rgb, ClickEvent, Div, ElementId, MouseButton, SharedString, WindowContext,
};

use crate::prelude::*;

#[derive(IntoElement)]
pub struct Button {
    pub(super) base: Div,
    id: ElementId,
    label: SharedString,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            label: label.into(),
            disabled: false,
            on_click: None,
        }
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .h_flex()
            .id(self.id.clone())
            .group("")
            .flex_none()
            .px_2()
            .bg(rgb(0xf7f7f7))
            .active(|this| this.opacity(0.85))
            .border_1()
            .border_color(rgb(0xe0e0e0))
            .rounded_md()
            .cursor_pointer()
            .child(self.label.clone())
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, move |_, window| window.prevent_default())
                        .on_click(move |event, cx| {
                            cx.stop_propagation();
                            (on_click)(event, cx);
                        })
                },
            )
    }
}
