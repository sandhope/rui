use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Assets, IconName, Root, Row, Text};
use strum::IntoEnumIterator;

struct IconStory;

impl Render for IconStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Text::new("All Icons").pb_3()

            Row! {}
            //.children(IconName::iter().map(Icon::new))
            .children(IconName::iter())
            .flex_wrap()
            .gap_3()
        }
        .justify_center()
        .items_center()
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        cx.activate(true);
        Theme::init(cx, None, None);
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
