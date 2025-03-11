use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Button, Root, Row, Text};

struct Tiles {
    text: SharedString,
}

impl Render for Tiles {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Text::new(self.text.clone())

            Row!{
                Text::new("Text")
                Text::new("Text")
                Text::new("Text")
            }
            .gap_4()

            Button::new("btn_id").text("Click Me").on_click(|_event, _cx, _app| {
                println!("clicked");
            })
        }
        .text_xl()
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
            |_, cx| {
                cx.new(|_| Tiles {
                    text: "Hello World!".into(),
                })
            },
        )
        .unwrap();
    });
}
