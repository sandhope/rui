use gpui::{
    prelude::*, px, rgb, size, App, Application, Bounds, Context, Window, WindowBounds,
    WindowOptions,
};

use rui::{prelude::*, Col, Row, Text};

struct Tiles;

impl Render for Tiles {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Row!{
                Text::new("row")
                Text::new("row")
                Text::new("row")
            }
            Row!{
                Text::new("row")
                Text::new("row")
                Text::new("row")
            }
            Text::new("Text")
            Text::new("Text").padding(10.0)
            Text::new("Text")
        }
        .flex()
        .bg(gpui::white())
        .size_full()
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0x000000))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Tiles {}),
        )
        .unwrap();
    });
}
