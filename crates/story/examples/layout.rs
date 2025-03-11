use gpui::{px, size, Application, Bounds, Context, FontWeight, WindowBounds, WindowOptions};

use rui::{prelude::*, Root, Row, Section, StyledExt, Text};

struct LayoutStory;

impl Render for LayoutStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Row!{
                Text::new("row").padding(0.).border_1().border_color(gpui::black())
                Text::new("row").padding((0., 10.0)).border_1().border_color(gpui::black())
                Text::new("row").padding((0.,0.,0.0,20.)).border_1().border_color(gpui::black())
            }
            Row!{
                Text::new("row").padding(0.).border_1().border_color(gpui::black())
                Text::new("row").padding_x(10.0).border_1().border_color(gpui::black())
                Text::new("row").padding_left(20.0).border_1().border_color(gpui::black())
            }
            Section!{
                Text::new("padding(custom title)").margin((-10., 0., 10., 0.)).font_weight(FontWeight::SEMIBOLD)
                Text::new("row").padding(0.).border_1().border_color(gpui::black())
                Text::new("row").padding_y(10.).border_1().border_color(gpui::black())
                Text::new("row").padding_left(20.).border_1().border_color(gpui::black())
            }
            Section!{
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
        cx.activate(true);
        Theme::init(cx, None, None);
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
