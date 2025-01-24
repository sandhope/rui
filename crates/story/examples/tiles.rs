use gpui::{div, prelude::*, rgb, App, AppContext, SharedString, ViewContext, WindowOptions};

use ui::{button::Button, label::Label};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0x000000))
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
                    .child(Button::new("Click Me", |_cx| {
                        println!("Button clicked");
                    }))
                    .child(Button::new("Click Me", |_cx| {
                        println!("Button clicked");
                    }))
                    .child(Button::new("Click Me", |_cx| {
                        println!("Button clicked");
                    })),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}
