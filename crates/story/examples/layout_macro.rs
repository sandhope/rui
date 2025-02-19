use gpui::{px, size, Application, Bounds, Context, FontWeight, WindowBounds, WindowOptions};

use rui::{prelude::*, StyledExt, Text};
use rui_macros::{root, row, section};

struct LayoutStory;

impl Render for LayoutStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        root! {
            row!{
                Text::new("row").padding(0.).border_1().border_color(gpui::black())
                Text::new("row").padding((0., 10.0)).border_1().border_color(gpui::black())
                Text::new("row").padding((0.,0.,0.0,20.)).border_1().border_color(gpui::black())
            }
            row!{
                Text::new("row").padding(0.).border_1().border_color(gpui::black())
                Text::new("row").padding_x(10.0).border_1().border_color(gpui::black())
                Text::new("row").padding_left(20.0).border_1().border_color(gpui::black())
            }
            section!{
                Text::new("padding(custom title)").margin((-10., 0., 10., 0.)).font_weight(FontWeight::SEMIBOLD)
                Text::new("row").padding(0.).border_1().border_color(gpui::black())
                Text::new("row").padding_y(10.).border_1().border_color(gpui::black())
                Text::new("row").padding_left(20.).border_1().border_color(gpui::black())
            }
            section!{
                "margin";
                Text::new("row").margin(0.).border_1().border_color(gpui::black())
                Text::new("row").margin_y(10.).border_1().border_color(gpui::black())
                Text::new("row").margin_left(20.).border_1().border_color(gpui::black())
            }

            Text::new("Text").border_1().border_color(gpui::black())
            Text::new("Text").padding(10.0).border_1().border_color(gpui::black())
            Text::new("Text").padding_left(20.).border_1().border_color(gpui::black())

            div()
                .flex()
                .gap_2()
                .child(div().size_8().bg(gpui::red()))
                .child(div().size_8().bg(gpui::green()))
                .child(div().size_8().bg(gpui::blue()))
                .child(div().size_8().bg(gpui::yellow()))
                .child(div().size_8().bg(gpui::black()))
                .child(div().size_8().bg(gpui::white()))
        }
        .justify_center()
        .items_start()
        .text_xl()
        .px_4()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        Theme::init(cx, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| LayoutStory {}),
        )
        .unwrap();
    });
}
