use gpui::{
    div, prelude::*, px, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

use ui::{Button, Col, Label, Row};

struct Tiles {
    text: SharedString,
}

impl Render for Tiles {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Row! {
            format!("Hello, {}!", &self.text)

            Col!{
                Label::new("Text")
                Label::new("Text")
                Label::new("Text")
            }
            .w_full()
            .gap_4()


            Col!{
                Label::new("Text")
                Label::new("Text")
                Label::new("Text")
            }
            .w_full()
            .gap_4()

            Col!{
                Button::new("btn_id", "Click Me").on_click(|_event, _cx, _app| {
                    println!("clicked");
                })
                Button::new("btn_id", "Click Me")
                Button::new("btn_id", "Click Me")
            }
            .w_full()
            .gap_4()

        }
        .bg(gpui::white())
        .size_full()
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(gpui::black())
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
            |_, cx| {
                cx.new(|_| Tiles {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
    });
}
