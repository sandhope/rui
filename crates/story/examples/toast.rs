use gpui::{size, Application, Bounds, WindowBounds, WindowOptions};
use rui::{prelude::*, Button, IconName, Root, StatusToast, Theme, ToastIcon};

struct ToastStory {}

impl Render for ToastStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Button::new("open-toast")
                .text("open-toast")
                .on_click(| _, window, cx| {
                    let status_toast =
                        StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                            this.icon(ToastIcon::new(IconName::Github))
                                .action("Open Pull Request", |_, cx| {
                                    cx.open_url("https://github.com/")
                                })
                        });
                    window.toggle_status_toast(cx, status_toast)
                })
            Button::new("open-close-toast")
                .text("open-close-toast")
                .on_click(| _, window, cx| {
                    let status_toast =
                        StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                            this.icon(ToastIcon::new(IconName::Github))
                                .action("Open Pull Request", |_, cx| {
                                    cx.open_url("https://github.com/")
                                })
                        });
                    window.toggle_status_toast(cx, status_toast)
                })

            Button::new("appearance")
                .text(cx.theme().appearance.to_string())
                .on_click(cx.listener(|_, _, window, cx| {
                    cx.theme_mut().toggle_builtin_appearance(window);
                }))
        }
        .p_4()
        .gap_1()
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        Theme::init(cx, None, None);

        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| {
                let view = cx.new(|_cx| ToastStory {});
                cx.new(|cx| Root::new(cx, view.into()))
            },
        )
        .unwrap();
    });
}
