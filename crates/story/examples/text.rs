use gpui::{
    px, rems, rgb, size, Application, Bounds, Context, FontWeight, WindowBounds, WindowOptions,
};

use rui::{prelude::*, Button, Col, Row, Section, Text};

struct TextStory {
    masked: bool,
}

impl Render for TextStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
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


            Section! {
                "Maksed Text";
                Col! {
                    Row!{
                        Text::new("9,182,1 USD").text_2xl().masked(self.masked)

                        Button::new("button_id", "btn-mask")
                        .on_click(cx.listener(|this, _, _window,_cx| {
                            this.masked = !this.masked;
                        }))
                    }

                    Text::new("500 USD").text_xl().masked(self.masked)
                }
                .w_full()
                .gap_4()
            }
        }
        .bg(gpui::white())
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
            |_, cx| cx.new(|_| TextStory { masked: false }),
        )
        .unwrap();
    });
}
