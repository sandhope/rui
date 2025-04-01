use std::rc::Rc;

use crate::{prelude::*, Button, Color, Icon, IconName, ToastAction, ToastView};
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement};

use super::Text;

#[derive(Clone, Copy)]
pub struct ToastIcon {
    icon: IconName,
    color: Hsla,
}

impl ToastIcon {
    pub fn new(icon: IconName) -> Self {
        Self {
            icon,
            color: Color::black(),
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }
}

impl From<IconName> for ToastIcon {
    fn from(icon: IconName) -> Self {
        Self {
            icon,
            color: Color::black(),
        }
    }
}

pub struct StatusToast {
    icon: Option<ToastIcon>,
    text: SharedString,
    action: Option<ToastAction>,
    this_handle: Entity<Self>,
    focus_handle: FocusHandle,
}

impl StatusToast {
    pub fn new(
        text: impl Into<SharedString>,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            f(
                Self {
                    text: text.into(),
                    icon: None,
                    action: None,
                    this_handle: cx.entity(),
                    focus_handle,
                },
                cx,
            )
        })
    }

    pub fn icon(mut self, icon: ToastIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        f: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        let this_handle = self.this_handle.clone();
        self.action = Some(ToastAction::new(
            label.into(),
            Some(Rc::new(move |window, cx| {
                this_handle.update(cx, |_, cx| {
                    cx.emit(DismissEvent);
                });
                f(window, cx);
            })),
        ));
        self
    }
}

impl Render for StatusToast {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("status-toast")
            .elevation_3(cx)
            .gap_2()
            .py_1p5()
            .px_2p5()
            .flex_none()
            .bg(cx.theme().colors.bg_elevated_surface)
            .shadow_lg()
            .items_center()
            .when_some(self.icon.as_ref(), |this, icon| {
                this.child(Icon::new(icon.icon).color(icon.color))
            })
            .child(Text::new(self.text.clone()))
            .when_some(self.action.as_ref(), |this, action| {
                this.child(
                    Button::new(action.id.clone())
                        .text(action.label.clone())
                        .color(Color::Primary)
                        .when_some(action.on_click.clone(), |el, handler| {
                            el.on_click(move |_click_event, window, cx| handler(window, cx))
                        }),
                )
            })
    }
}

impl ToastView for StatusToast {
    fn action(&self) -> Option<ToastAction> {
        self.action.clone()
    }
}

impl Focusable for StatusToast {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for StatusToast {}
