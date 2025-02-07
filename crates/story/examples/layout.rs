use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Window, WindowBounds,
    WindowOptions,
};

use ui::{Row, StyledExt, Text};
use ui_macros::{col, row};

struct Layout;

impl Render for Layout {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        col! {
            row!{
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
            Text::new("Text").padding(ui::Padding::All, px(10.0))
            Text::new("Text")
        }
        .flex()
        .bg(rgb(0xffffff))
        .size_full()
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0x000000))
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
            |_, cx| cx.new(|_| Layout {}),
        )
        .unwrap();
    });
}
