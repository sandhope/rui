use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Root, Theme};

struct CardStory;

impl Render for CardStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Card::new()
                .child("Vertical")

            Card::new()
                .direction_horizontal()
                .child("horizontal")
        }
        .p_4()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| CardStory {}),
        )
        .unwrap();
    });
}
