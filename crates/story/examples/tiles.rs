use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

use ui::{Button, Label};

struct Tiles {
    text: SharedString,
}

impl Render for Tiles {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0x000000))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .gap_4()
                    .child(Label::new("Text"))
                    .child(Label::new("Text"))
                    .child(Label::new("Text")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .gap_4()
                    .child(Label::new("Text"))
                    .child(Label::new("Text"))
                    .child(Label::new("Text")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .gap_4()
                    .child(
                        Button::new("btn_id", "Click Me").on_click(|_event, _cx, _app| {
                            println!("clicked");
                        }),
                    )
                    .child(Button::new("btn_id", "Click Me"))
                    .child(Button::new("btn_id", "Click Me")),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Tiles {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
    });
}
