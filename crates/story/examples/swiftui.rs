use gpui::{div, prelude::*, rgb, App, AppContext, ViewContext, WindowOptions};

use ui::{Col, Label};

struct Tiles;

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
            .child(Col! {
                Label::new("Text")
                Label::new("Text")
                Label::new("Text")
            })
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| cx.new_view(|_cx| Tiles {}))
            .unwrap();
    });
}
