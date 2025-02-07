use gpui::{
    div, prelude::*, px, rems, rgb, size, App, Application, Bounds, Context, FontWeight, Window,
    WindowBounds, WindowOptions,
};

use ui::{h_flex, v_flex, Button, Col, Label};

use story::section;

struct LabelStory {
    masked: bool,
}

impl Render for LabelStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
        .size_full()
        .bg(rgb(0xffffff))
        .gap_6()
        .child(
            section("Label", window)
                .items_start()
                .child(
                    Col!{
                        Label::new("Text align left")
                        Label::new("Text align center").text_center()
                        Label::new("Text align right").text_right()
                    }
                    .w_full()
                    .gap_4()
                )
                .child(Label::new("Color Label").bg(gpui::blue()).text_color(gpui::red()))
                .child(Label::new("Font Size Label").text_size(px(20.)).font_weight(FontWeight::SEMIBOLD).line_height(rems(1.8)))
                .child(
                    div().w(px(200.)).child(
                        Label::new("Label should support text wrap in default, if the text is too long, it should wrap to the next line.")
                            .line_height(rems(1.8)),
                    ),
                ),
        )
        .child(
            section("Maksed Label", window).child(
                v_flex()
                    .w_full()
                    .gap_4()
                    .child(
                        h_flex().child(Label::new("9,182,1 USD").text_2xl().masked(self.masked)).child(
                            // Button::new("btn-mask", |_cx| {
                            //     self.masked = !self.masked;
                            // })
                            Button::new("button_id", "btn-mask")
                            .on_click(cx.listener(|this, _, _window,_cx| {
                                this.masked = !this.masked;
                            })),
                        ),
                    )
                    .child(Label::new("500 USD").text_xl().masked(self.masked)),
            ),
        )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
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
