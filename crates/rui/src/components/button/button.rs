use crate::{prelude::*, Color, Icon, Size, Text};
use gpui::{AnyElement, AnyView, ClickEvent, ElementId, MouseButton};

#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    text: Option<SharedString>,
    icon: Option<Icon>,
    size: Size,
    disabled: bool,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
    variant: ButtonVariant,
    color: ButtonColor,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Solid,
    Soft,
    Outline,
    Ghost,
    Plain,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
    Default,
    Primary,
    Secondary,
    Info,
    Success,
    Warning,
    Danger,
    Custom(Hsla),
}

impl ButtonColor {
    fn hsla(&self, cx: &App) -> Hsla {
        match self {
            Self::Default => cx.theme().colors.default,
            Self::Primary => cx.theme().colors.primary,
            Self::Secondary => cx.theme().colors.secondary,
            Self::Info => cx.theme().colors.info,
            Self::Success => cx.theme().colors.success,
            Self::Warning => cx.theme().colors.warning,
            Self::Danger => cx.theme().colors.danger,
            Self::Custom(color) => *color,
        }
    }
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
            variant: ButtonVariant::Solid,
            color: ButtonColor::Default,
        }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
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

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    // default
    // pub fn solid(mut self) -> Self {
    //     self.variant = ButtonVariant::Solid;
    //     self
    // }
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

    pub fn color(mut self, color: ButtonColor) -> Self {
        self.color = color;
        self
    }
    pub fn primary(mut self) -> Self {
        self.color = ButtonColor::Primary;
        self
    }
    pub fn secondary(mut self) -> Self {
        self.color = ButtonColor::Secondary;
        self
    }
    pub fn info(mut self) -> Self {
        self.color = ButtonColor::Info;
        self
    }
    pub fn success(mut self) -> Self {
        self.color = ButtonColor::Success;
        self
    }
    pub fn warning(mut self) -> Self {
        self.color = ButtonColor::Warning;
        self
    }
    pub fn danger(mut self) -> Self {
        self.color = ButtonColor::Danger;
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
            .items_center()
            .text_center()
            .overflow_hidden()
            .gap(rems_from_px(4.))
            .rounded_md()
            .map(|this| match self.variant {
                ButtonVariant::Solid => this,
                ButtonVariant::Soft => this,
                ButtonVariant::Outline => this.border_1().border_color(cx.theme().colors.border),
                ButtonVariant::Ghost => this,
                ButtonVariant::Plain => this,
            })
            .map(|this| match self.color {
                ButtonColor::Default => this
                    .bg(cx.theme().colors.default)
                    .text_color(cx.theme().colors.bg),
                ButtonColor::Primary => this
                    .bg(cx.theme().colors.primary)
                    .text_color(cx.theme().colors.bg),
                ButtonColor::Secondary => this
                    .bg(cx.theme().colors.secondary)
                    .text_color(cx.theme().colors.bg),
                ButtonColor::Info => this
                    .bg(cx.theme().colors.secondary)
                    .text_color(cx.theme().colors.bg),
                ButtonColor::Success => this
                    .bg(cx.theme().colors.success)
                    .text_color(cx.theme().colors.bg),
                ButtonColor::Warning => this
                    .bg(cx.theme().colors.warning)
                    .text_color(cx.theme().colors.bg),
                ButtonColor::Danger => this
                    .bg(cx.theme().colors.danger)
                    .text_color(cx.theme().colors.bg),
                ButtonColor::Custom(color) => this.bg(color),
            })
            .map(|this| match self.size {
                Size::XSmall => this.h_7().px_2p5(),
                Size::Small => this.h_8().px_3p5(),
                Size::Medium => this.h_9().px_4(),
                Size::Large => this.h_10().px_5(),
                Size::Custom(size) => this.w(size).h(size * 0.1 + rems(1.5)),
            })
            .when(self.disabled, |this| this.cursor_not_allowed())
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|this| match self.variant {
                        ButtonVariant::Solid => this.opacity(0.98),
                        ButtonVariant::Soft => this.opacity(0.9),
                        ButtonVariant::Outline => this.opacity(0.9),
                        ButtonVariant::Ghost => this.opacity(0.9),
                        ButtonVariant::Plain => this.opacity(0.9),
                    })
                    .active(|this| this)
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
        Self::new(text).text(text)
    }
}

impl From<SharedString> for Button {
    fn from(text: SharedString) -> Self {
        Self::new(text.clone()).text(text)
    }
}

impl From<String> for Button {
    fn from(text: String) -> Self {
        Self::new(SharedString::from(text.clone())).text(text)
    }
}
