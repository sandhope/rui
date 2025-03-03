use crate::{prelude::*, Icon, Size, Text};
use gpui::{AnyElement, AnyView, ClickEvent, ElementId, MouseButton};

#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    text: Option<Text>,
    icon: Option<Icon>,
    size: Size,
    disabled: bool,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            text: None,
            icon: None,
            size: Size::default(),
            disabled: false,
            on_click: None,
            tooltip: None,
            children: Vec::new(),
        }
    }

    pub fn text(mut self, text: impl Into<Text>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .flex()
            .id(self.id.clone())
            .group("")
            .justify_center()
            .text_center()
            .overflow_hidden()
            .gap(rems_from_px(4.))
            .border_1()
            .border_color(cx.theme().colors.border)
            .map(|this| match self.size {
                Size::XSmall => this.h_5().px(rems_from_px(3.)),
                Size::Small => this.h_6().px(rems_from_px(4.)),
                Size::Medium => this.h_7().px(rems_from_px(5.)),
                Size::Large => this.h_8().px(rems_from_px(6.)),
                Size::Custom(size) => this.px(size),
            })
            .when(self.disabled, |this| this.cursor_not_allowed())
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|this| this.bg(cx.theme().colors.bg))
                    .active(|this| this.bg(cx.theme().colors.bg))
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            (on_click)(event, window, cx)
                        })
                },
            )
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |window, cx| tooltip(window, cx))
            })
            .when_some(self.icon, |this, icon| this.child(icon))
            .when_some(self.text, |this, text| this.child(text))
            .children(self.children)
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements)
    }
}

impl From<&'static str> for Button {
    fn from(text: &'static str) -> Self {
        Self::new(text).text(Text::new(text))
    }
}

impl From<SharedString> for Button {
    fn from(text: SharedString) -> Self {
        Self::new(text.clone()).text(Text::new(text))
    }
}

impl From<String> for Button {
    fn from(text: String) -> Self {
        Self::new(SharedString::from(text.clone())).text(Text::new(text))
    }
}
