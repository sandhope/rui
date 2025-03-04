use gpui::{px, rems, size, Application, Bounds, Context, FontWeight, WindowBounds, WindowOptions};

use rui::{prelude::*, Root, Row, Section, Text};

struct TextStory;

impl Render for TextStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Section! {
                "label";
                Row!{
                    Text::new("row")
                    Text::new("row")
                    Text::new("row")
                }
                .gap_1()
                Text::new("Text").bg(gpui::blue()).text_color(gpui::red())
                Text::new("Font Size Label")
                    .text_size(px(20.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .line_height(rems(1.8))
                    .padding(10.0)
                Text::new("Text").padding_left(20.)

                div().w(px(200.)).child(
                    Text::new("Label should support text wrap in default, if the text is too long, it should wrap to the next line.")
                        .line_height(rems(1.8)),
                )
            }
        }
        .px_4()
        .text_xl()
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
            |_, cx| cx.new(|_| TextStory {}),
        )
        .unwrap();
    });
}
