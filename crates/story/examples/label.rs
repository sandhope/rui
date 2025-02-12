use gpui::{
    px, rems, rgb, size, Application, Bounds, Context, FontWeight, WindowBounds, WindowOptions,
};

use rui::{prelude::*, Button, Col, Label, Row, Section};

struct LabelStory {
    masked: bool,
}

impl Render for LabelStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col!{
            Section! {
                "Label";
                Col!{
                    Label::new("Text align left")
                    Label::new("Text align center").text_center()
                    Label::new("Text align right").text_right()

                    Label::new("Color Label")
                        .bg(gpui::blue())
                        .text_color(gpui::red())

                    Label::new("Font Size Label")
                        .text_size(px(20.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .line_height(rems(1.8))

                    div().w(px(200.)).child(
                        Label::new("Label should support text wrap in default, if the text is too long, it should wrap to the next line.")
                            .line_height(rems(1.8)),
                    )
                }
                .w_full()
                .gap_4()
            }
            .items_start()

            Section! {
                "Maksed Label";
                Col! {
                    Row!{
                        Label::new("9,182,1 USD").text_2xl().masked(self.masked)

                        Button::new("button_id", "btn-mask")
                        .on_click(cx.listener(|this, _, _window,_cx| {
                            this.masked = !this.masked;
                        }))
                    }

                    Label::new("500 USD").text_xl().masked(self.masked)
                }
                .w_full()
                .gap_4()
            }
        }
        .size_full()
        .bg(rgb(0xffffff))
        .gap_6()
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
            |_, cx| cx.new(|_| LabelStory { masked: true }),
        )
        .unwrap();
    });
}
