use gpui::{px, rgb, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Col};

struct CardStory;

impl Render for CardStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Card::new()
                .child("Vertical")

            Card::new()
                .direction_horizontal()
                .child("horizontal")
        }
        .bg(cx.theme().appearance.bg_color())
        .size_full()
        .text_xl()
        .text_color(rgb(0x000000))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        rui::init(cx);
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
