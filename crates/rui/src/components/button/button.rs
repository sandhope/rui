use crate::{prelude::*, Color, Icon, IconName, Size, Text};
use gpui::{
    linear, percentage, Animation, AnimationExt as _, AnyElement, AnyView, ClickEvent, Corners,
    Edges, ElementId, MouseButton, Transformation,
};
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Solid,
    Soft,
    Surface,
    Outline,
    Ghost,
    Plain,
}

// use std::fmt::{self, Display, Formatter};
// impl Display for ButtonVariant {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match self {
//             Self::Solid => write!(f, "Solid"),
//             Self::Soft => write!(f, "Soft"),
//             Self::Surface => write!(f, "Surface"),
//             Self::Outline => write!(f, "Outline"),
//             Self::Ghost => write!(f, "Ghost"),
//             Self::Plain => write!(f, "Plain"),
//         }
//     }
// }

#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    text: Option<Text>,
    icon: Option<Icon>,
    icon_right: bool,
    loading_icon: Icon,
    size: Size,
    disabled: bool,
    loading: bool,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
    variant: ButtonVariant,
    color: Color,
    border_corners: Corners<bool>,
    border_edges: Edges<bool>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            text: None,
            icon: None,
            icon_right: false,
            loading_icon: Icon::new(IconName::Loading),
            size: Size::default(),
            disabled: false,
            loading: false,
            on_click: None,
            tooltip: None,
            children: Vec::new(),
            variant: ButtonVariant::Solid,
            color: Color::Default,
            border_corners: Corners::all(true),
            border_edges: Edges::all(true),
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

    pub fn icon_right(mut self) -> Self {
        self.icon_right = true;
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

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
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
    pub fn solid(mut self) -> Self {
        self.variant = ButtonVariant::Solid;
        self
    }
    pub fn soft(mut self) -> Self {
        self.variant = ButtonVariant::Soft;
        self
    }
    pub fn surface(mut self) -> Self {
        self.variant = ButtonVariant::Surface;
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

    pub fn loading_icon(mut self, icon: impl Into<Icon>) -> Self {
        self.loading_icon = icon.into();
        self
    }

    /// Set the border corners side of the Button.
    pub fn border_corners(mut self, corners: impl Into<Corners<bool>>) -> Self {
        self.border_corners = corners.into();
        self
    }

    /// Set the border edges of the Button.
    pub fn border_edges(mut self, edges: impl Into<Edges<bool>>) -> Self {
        self.border_edges = edges.into();
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
        let is_light = cx.theme().appearance.is_light();
        let bg = cx.theme().colors.bg;
        let color = self.color.hsla(cx);
        let outline_color = color.opacity(0.8);
        let soft_color = color.soft();
        // let soft_color = color.opacity(0.3);
        let text_color = match self.variant {
            ButtonVariant::Solid => bg,
            _ => color,
        };
        let loading_icon = self.loading_icon.with_animation(
            "loading",
            Animation::new(Duration::from_millis(800))
                .repeat()
                .with_easing(linear),
            |this, delta| this.transform(Transformation::rotate(percentage(delta))),
        );

        self.base
            .flex()
            .id(self.id.clone())
            .group("")
            .justify_center()
            .items_center()
            .text_center()
            .overflow_hidden()
            .gap_2()
            .text_color(text_color)
            .when(self.border_edges.left, |this| this.border_l_1())
            .when(self.border_edges.right, |this| this.border_r_1())
            .when(self.border_edges.top, |this| this.border_t_1())
            .when(self.border_edges.bottom, |this| this.border_b_1())
            .when(self.border_corners.top_left, |this| this.rounded_tl_md())
            .when(self.border_corners.bottom_left, |this| this.rounded_bl_md())
            .when(self.border_corners.top_right, |this| this.rounded_tr_md())
            .when(self.border_corners.bottom_right, |this| {
                this.rounded_br_md()
            })
            .map(|this| match self.variant {
                ButtonVariant::Solid => this.bg(color),
                ButtonVariant::Soft => this.bg(soft_color),
                ButtonVariant::Surface => this.bg(soft_color).border_color(color.opacity(0.4)),
                ButtonVariant::Outline => this.border_color(outline_color),
                ButtonVariant::Ghost | ButtonVariant::Plain => this,
            })
            .map(|this| {
                if self.text.is_none() && self.children.is_empty() {
                    match self.size {
                        Size::XSmall => this.size_7(),
                        Size::Small => this.size_8(),
                        Size::Medium => this.size_9(),
                        Size::Large => this.size_10(),
                        Size::Custom(size) => this.size(size),
                    }
                } else {
                    match self.size {
                        Size::XSmall => this.h_7().px_2p5(),
                        Size::Small => this.h_8().px_3p5(),
                        Size::Medium => this.h_9().px_4(),
                        Size::Large => this.h_10().px_5(),
                        Size::Custom(size) => this.w(size).h(size * 0.1 + rems(1.5)),
                    }
                }
            })
            .when(self.disabled, |this| this.cursor_not_allowed().opacity(0.5))
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|this| match self.variant {
                        ButtonVariant::Solid => {
                            this.bg(color.opacity(if is_light { 0.9 } else { 0.7 }))
                        }
                        ButtonVariant::Soft => this.bg(color.opacity(0.6)),
                        ButtonVariant::Surface => this.bg(color.opacity(0.6)),
                        ButtonVariant::Outline => this.bg(soft_color),
                        ButtonVariant::Ghost => this.bg(soft_color),
                        ButtonVariant::Plain => this.opacity(0.9),
                    })
                    .active(|this| match self.variant {
                        ButtonVariant::Solid => this.bg(color.opacity(1.)),
                        _ => this,
                    })
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
            // .when(self.loading, |this| {
            //     this.child(loading_icon)
            //         .when_some(self.text.clone(), |this, text| this.child(text))
            // })
            // .when(!self.loading, |this| {
            //     // this.when(self.icon_right, |this| {
            //     //     this.when_some(self.text.clone(), |this, text| this.child(text))
            //     // })
            //     // .when_some(self.icon, |this, icon| this.child(icon.color(icon_color)))
            //     // .when(!self.icon_right, |this| {
            //     //     this.when_some(self.text, |this, text| this.child(text))
            //     // })
            //     this.map(|this| match self.icon_right {
            //         true => this
            //             .when_some(self.text, |this, text| this.child(text))
            //             .when_some(self.icon, |this, icon| this.child(icon.color(icon_color))),
            //         false => this
            //             .when_some(self.icon, |this, icon| this.child(icon.color(icon_color)))
            //             .when_some(self.text, |this, text| this.child(text)),
            //     })
            // })
            .map(|this| match self.loading {
                true => this
                    .child(loading_icon)
                    .when_some(self.text, |this, text| this.child(text)),
                false => this.map(|this| match self.icon_right {
                    true => this
                        .when_some(self.text, |this, text| this.child(text))
                        .when_some(self.icon, |this, icon| this.child(icon)),
                    false => this
                        .when_some(self.icon, |this, icon| this.child(icon))
                        .when_some(self.text, |this, text| this.child(text)),
                }),
            })
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
