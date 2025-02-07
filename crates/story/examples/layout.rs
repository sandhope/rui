use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Window, WindowBounds,
    WindowOptions,
};

use ui::{Col, Padding, Row, StyledExt, Text};
use ui_macros::{col, row};

struct Layout;

impl Render for Layout {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        col! {
            row!{
                Text::new("row").padding(Padding::Left(0.0))
                Text::new("row").padding((0.,10.0))
                Text::new("row").padding((0.,0.,0.0,20.))
            }
            Row!{
                Text::new("row")
                Text::new("row")
                Text::new("row")
            }
            Col!{
                Text::new("row").padding(0.0)
                Text::new("row").padding((0.0, 10.0))
                Text::new("row").padding((0., 0.,0.,20.))
            }

            Text::new("Text")
            Text::new("Text").padding(10.0)
            Text::new("Text").padding(Padding::Left(20.0))
        }
        .flex()
        .bg(rgb(0xffffff))
        .size_full()
        .justify_center()
        .items_start()
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
            |_, cx| cx.new(|_| Layout {}),
        )
        .unwrap();
    });
}
