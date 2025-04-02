use crate::{prelude::*, ModalLayer, ModalView, ToastLayer, ToastView};
use gpui::{AnyView, Entity};

pub struct Root {
    toast_layer: Entity<ToastLayer>,
    modal_layer: Entity<ModalLayer>,
    view: AnyView,
}

impl Root {
    pub fn new(cx: &mut App, view: AnyView) -> Self {
        let toast_layer = cx.new(|_| ToastLayer::new());
        let modal_layer = cx.new(|_| ModalLayer::new());

        Self {
            toast_layer,
            modal_layer,
            view,
        }
    }

    pub fn toggle_modal<V: ModalView, B>(&mut self, window: &mut Window, cx: &mut App, build: B)
    where
        B: FnOnce(&mut Window, &mut Context<V>) -> V,
    {
        self.modal_layer.update(cx, |modal_layer, cx| {
            modal_layer.toggle_modal(window, cx, build)
        })
    }

    pub fn toggle_status_toast<V: ToastView>(&mut self, entity: Entity<V>, cx: &mut App) {
        self.toast_layer
            .update(cx, |toast_layer, cx| toast_layer.toggle_toast(cx, entity))
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .text_color(cx.theme().colors.text)
            .bg(cx.theme().colors.bg)
            .child(self.view.clone())
            .child(self.toast_layer.clone())
            .child(self.modal_layer.clone())
    }
}
