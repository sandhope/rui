use gpui::{size, Application, Bounds, Entity, ManagedView, WindowBounds, WindowOptions};
use rui::{
    prelude::*, AlertModal, Button, Color, IconName, Modal, ModalLayer, ModalView, Root,
    StatusToast, Text, Theme, ToastIcon, ToastLayer, ToastView,
};

struct AlertModalStory {
    toast_layer: Entity<ToastLayer>,
    modal_layer: Entity<ModalLayer>,
}

impl AlertModalStory {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let toast_layer = cx.new(|_| ToastLayer::new());
        let modal_layer = cx.new(|_| ModalLayer::new());

        AlertModalStory {
            toast_layer,
            modal_layer,
        }
    }

    pub fn has_active_modal(&self, _: &mut Window, cx: &mut App) -> bool {
        self.modal_layer.read(cx).has_active_modal()
    }

    pub fn active_modal<V: ManagedView + 'static>(&self, cx: &App) -> Option<Entity<V>> {
        self.modal_layer.read(cx).active_modal()
    }

    pub fn toggle_modal<V: ModalView, B>(&mut self, window: &mut Window, cx: &mut App, build: B)
    where
        B: FnOnce(&mut Window, &mut Context<V>) -> V,
    {
        self.modal_layer.update(cx, |modal_layer, cx| {
            modal_layer.toggle_modal(window, cx, build)
        })
    }

    pub fn toggle_status_toast<V: ToastView>(&mut self, cx: &mut App, entity: Entity<V>) {
        self.toast_layer
            .update(cx, |toast_layer, cx| toast_layer.toggle_toast(cx, entity))
    }
}

impl Render for AlertModalStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            AlertModal::new("default-modal", "default")
                .child("Default style.")

            AlertModal::new("simple-modal", "Do you want to leave the current call?")
                .child("The current window will be closed, and connections to any shared projects will be terminated.")
                .primary_button("Leave Call")

            AlertModal::new("custom-modal", "custom-modal")
                .child("Custom modal.")
                .primary_button(Button::new("yes-btn").text("Yes").primary().size(Size::Small))
                .dismiss_button(Button::new("no-btn").text("No").danger().size(Size::Small))

            AlertModal::new("custom-footer", "custom-footer")
                .child("Custom footer.")
                .header(h_flex()
                    .items_center()
                    .justify_between()
                    .child(Text::new("title").color(Color::blue()))
                    .child(Button::new("id").icon(IconName::X).ghost().size(Size::XSmall))
                    .into_any_element()
                )
                .footer(h_flex()
                    .h(rems(1.75))
                    .items_center()
                    .justify_center()
                    .pt_4()
                    .child(
                        Button::new("ok-btn-id")
                            .text("OK")
                            .size(Size::Small)
                            .surface()
                            .secondary()
                            .into_any_element()
                    )
                    .into_any_element())

            Button::new("open-modal")
                .text("open-modal")
                .on_click(cx.listener(|this, _, window, cx| {
                    this.toggle_modal(window,cx,|_window, cx| {
                        Modal::new(cx,"title").show_close()
                    })
                }))

            Row! {
                Button::new("open-toast")
                    .text("open-toast")
                    .on_click(cx.listener(|this, _, _window, cx| {
                        let status_toast =
                            StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                                this.icon(ToastIcon::new(IconName::Github))
                                    .action("Open Pull Request", |_, cx| {
                                        cx.open_url("https://github.com/")
                                    })
                            });
                        this.toggle_status_toast(cx, status_toast)
                    }))
                Button::new("open-close-toast")
                    .text("open-close-toast")
                    .on_click(cx.listener(|this, _, _window, cx| {
                        let status_toast =
                            StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                                this.icon(ToastIcon::new(IconName::Github))
                                    .action("Open Pull Request", |_, cx| {
                                        cx.open_url("https://github.com/")
                                    })
                            });
                        this.toggle_status_toast(cx, status_toast)
                    }))
            }

            Button::new("appearance")
                .text(cx.theme().appearance.to_string())
                .on_click(cx.listener(|_, _, window, cx| {
                    cx.theme_mut().toggle_builtin_appearance(window);
                }))
        }
        .p_4()
        .gap_1()
        .child(self.toast_layer.clone())
        .child(self.modal_layer.clone())
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        Modal::bind_keys(cx);

        cx.activate(true);
        Theme::init(cx, None, None);

        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| AlertModalStory::new(cx)),
        )
        .unwrap();
    });
}
