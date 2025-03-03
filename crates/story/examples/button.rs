use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Button, ButtonGroup, Root, Theme};

struct ButtonStory;

impl Render for ButtonStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Row! {
                Button::new("id")
                    .text("Click me")
                    .on_click(|_,_,_| println!("Button clicked!"))
            }

            Button::new("id")
                .text("Click me")
                .on_click(|_,_,_| println!("Button clicked!"))

            ButtonGroup::new().children(vec!["One","Two","Three"]).on_click(|v,_,_| println!("Button clicked!{}",v))
        }
        .p_4()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ButtonStory {}),
        )
        .unwrap();
    });
}
