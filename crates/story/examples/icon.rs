use gpui::{px, rgb, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Assets, Col, Icon, IconName, Row, Text};
use strum::IntoEnumIterator;

struct IconStory;

impl Render for IconStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Row! {
                Text::new("All Icons")
            }
            .padding_bottom(10.)

            Row! {}
            .children(IconName::iter().map(Icon::new))
            .flex_wrap()
            .gap_3()
        }
        .bg(gpui::white())
        .size_full()
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0x000000))
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| IconStory {}),
        )
        .unwrap();
    });
}
