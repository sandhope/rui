use gpui::{px, rgb};
use rui::{prelude::*, Col, Color, Row, Text};

struct ColorStory {
    title: SharedString,
}

impl Render for ColorStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Row! {
                Text::new(self.title.clone())
            }
            Row! {
                div().size_8().bg(Color::red())
                div().size_8().bg(Color::green())
                div().size_8().bg(Color::gray())
                div().size_8().bg(Color::blue())
                div().size_8().bg(Color::teal())
                div().size_8().bg(Color::pink())
                div().size_8().bg(Color::purple())
                div().size_8().bg(Color::cyan())
                div().size_8().bg(Color::orange())
                div().size_8().bg(Color::yellow())
                div().size_8().bg(Color::black())
                div().size_8().bg(Color::white())
            }
            Row! {
                div().size_8().bg(gpui::red())
                div().size_8().bg(gpui::green())
                div().size_8().bg(gpui::blue())
                div().size_8().bg(gpui::yellow())
                div().size_8().bg(gpui::black())
                div().size_8().bg(gpui::white())
                div().size_8().bg(gpui::rgb(0xffffff))
                div().size_8().bg(gpui::rgba(0xffffff7f))
                div().size_8().bg(gpui::rgb(0xffffff)).opacity(0.5)
                div().size_8().bg(rui::rgb(255, 255, 255)).opacity(0.5)
                div().size_8().bg(rui::rgba(255, 255, 255, 0.5))
                div().size_8().bg(gpui::hsla(0., 0., 1., 0.5))
            }
            .gap_1()
        }
        .gap_3()
        .bg(rgb(0x505050))
        .size(px(600.0))
        .justify_center()
        .items_center()
        .shadow_lg()
        .border_1()
        .border_color(rgb(0x0000ff))
        .text_xl()
        .text_color(rgb(0xffffff))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(600.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| ColorStory {
                    title: "Color exmaples!".into(),
                })
            },
        )
        .unwrap();
    });
}
