use gpui::{div, prelude::*, px, rgb, App, AppContext, ViewContext, WindowOptions};

use ui::{Col, Row, StyledExt, Text};

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
            .child(
                Col! {
                    Row!(
                        Text::new("row")
                        Text::new("row")
                        Text::new("row")
                    )
                    Row!(
                        Text::new("row")
                        Text::new("row")
                        Text::new("row")
                    )
                    Text::new("Text")
                    Text::new("Text").padding(ui::Padding::All, px(10.0))
                    Text::new("Text")
                }
                .px_8(),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| cx.new_view(|_cx| Tiles {}))
            .unwrap();
    });
}
