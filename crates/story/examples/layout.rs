use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Window, WindowBounds,
    WindowOptions,
};

use ui::{Col, Row, StyledExt, Text};
use ui_macros::{col, row};

struct Layout;

impl Render for Layout {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        col! {
            row!{
                Text::new("row").padding(0.).border_1().border_color(rgb(0x000000))
                Text::new("row").padding((0., 10.0)).border_1().border_color(rgb(0x000000))
                Text::new("row").padding((0.,0.,0.0,20.)).border_1().border_color(rgb(0x000000))
            }
            Row!{
                Text::new("row").padding(0.).border_1().border_color(rgb(0x000000))
                Text::new("row").padding_x(10.0).border_1().border_color(rgb(0x000000))
                Text::new("row").padding_left(20.0).border_1().border_color(rgb(0x000000))
            }
            Col!{
                Text::new("row").padding(0.).border_1().border_color(rgb(0x000000))
                Text::new("row").padding_y(10.).border_1().border_color(rgb(0x000000))
                Text::new("row").padding_left(20.).border_1().border_color(rgb(0x000000))
            }
            Col!{
                Text::new("row").margin(0.).border_1().border_color(rgb(0x000000))
                Text::new("row").margin_y(10.).border_1().border_color(rgb(0x000000))
                Text::new("row").margin_left(20.).border_1().border_color(rgb(0x000000))
            }

            Text::new("Text").border_1().border_color(rgb(0x000000))
            Text::new("Text").padding(10.0).border_1().border_color(rgb(0x000000))
            Text::new("Text").padding_left(20.).border_1().border_color(rgb(0x000000))
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
