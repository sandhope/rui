use crate::{prelude::*, ActiveTheme};
use gpui::{ClickEvent, MouseButton, Stateful};

#[derive(IntoElement)]
pub struct Link {
    base: Stateful<Div>,
    href: Option<SharedString>,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Link {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id).h_flex(),
            href: None,
            on_click: None,
            disabled: false,
        }
    }

    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for Link {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Link {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.base.extend(elements)
    }
}

impl RenderOnce for Link {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let color = cx.theme().colors.primary;

        self.base
            .cursor_pointer()
            .gap(rems_from_px(1.))
            .text_color(color)
            .text_decoration_1()
            .text_decoration_color(color)
            .hover(|this| this.opacity(0.8))
            //.active(|this| this.text_color(color.opacity(0.6)))
            .on_mouse_down(MouseButton::Left, |_, window, _cx| window.prevent_default())
            .on_click(move |event, window, cx| {
                cx.stop_propagation();
                if let Some(href) = &self.href {
                    cx.open_url(href);
                }
                if let Some(on_click) = &self.on_click {
                    on_click(event, window, cx);
                }
            })
    }
}
