use gpui::{size, Application, Bounds, WindowBounds, WindowOptions};
use rui::{prelude::*, AlertModal, Button, Color, IconName, Root, Text, Theme};

struct AlertModalStory;

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
                .on_click(cx.listener(|_, _, window, cx| {
                    cx.focus_handle().focus(window)
                }))

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
        cx.activate(true);
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AlertModalStory {}),
        )
        .unwrap();
    });
}
