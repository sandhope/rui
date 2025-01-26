use gpui::{
    div, prelude::*, px, rems, rgb, App, AppContext, FontWeight, ViewContext, WindowOptions,
};

use ui::{button::Button, h_flex, label::Label, v_flex};

use story::section;

struct LabelStory {
    masked: bool,
}

impl Render for LabelStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
        .size_full()
        .bg(rgb(0xffffff))
        .gap_6()
        .child(
            section("Label", cx)
                .items_start()
                .child(
                    v_flex()
                        .w_full()
                        .gap_4()
                        .child(Label::new("Text align left"))
                        .child(Label::new("Text align center").text_center())
                        .child(Label::new("Text align right").text_right()),
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
            section("Maksed Label", cx).child(
                v_flex()
                    .w_full()
                    .gap_4()
                    .child(
                        h_flex().child(Label::new("9,182,1 USD").text_2xl().masked(self.masked)).child(
                            // Button::new("btn-mask", |_cx| {
                            //     self.masked = !self.masked;
                            // })
                            Button::new("button_id", "btn-mask")
                            .on_click(cx.listener(|this, _, _| {
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
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| LabelStory { masked: true })
        })
        .unwrap();
    });
}
