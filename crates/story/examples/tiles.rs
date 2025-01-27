use gpui::{div, prelude::*, rgb, App, AppContext, SharedString, ViewContext, WindowOptions};

use ui::{Button, Label};

struct Tiles {
    text: SharedString,
}

impl Render for Tiles {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                    .child(Button::new("btn_id", "Click Me").on_click(|_event, _cx| {
                        println!("clicked");
                    }))
                    .child(Button::new("btn_id", "Click Me"))
                    .child(Button::new("btn_id", "Click Me")),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Tiles {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}
