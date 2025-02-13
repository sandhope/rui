use gpui::{
    black, px, rems, rgb, size, Application, Bounds, Context, FontWeight, WindowBounds,
    WindowOptions,
};

use rui::{prelude::*, Assets, Button, Col, Icon, IconName, Label, Row, Section, Text};

struct LabelStory {
    masked: bool,
}

impl Render for LabelStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Section! {
                "Label row";
                Row!{
                    Label::new(IconName::Eye, "simple")
                    Label::new(IconName::Eye, "icon right")
                        .icon_right()
                        .border_1()
                        .border_color(gpui::black())
                    Label::new(Icon::new(IconName::Bell), "icon new")
                    Label::new(Icon::new(IconName::Ai).color(gpui::blue()), "icon color")

                    Label::new(IconName::AiOpenAi, Text::new("text new"))
                    Label::new(IconName::AiOpenAi, Text::new("text color").text_color(gpui::red()).bg(black().opacity(0.5)))

                    Label::new(
                        Icon::new(IconName::Ai).color(gpui::blue()),
                        Text::new("complex")
                            .text_color(gpui::red())
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                    )
                }
                .gap_4()
            }

            Section! {
                "Label col";
                Col!{
                    Label::new(IconName::Eye, "simple")
                    Label::new(IconName::Eye, "icon right")
                        .icon_right()
                        .border_1()
                        .border_color(gpui::black())
                    Label::new(Icon::new(IconName::Bell), "icon new")
                    Label::new(Icon::new(IconName::Ai).color(gpui::blue()), "icon color")

                    Label::new(IconName::AiOpenAi, Text::new("text new"))
                    Label::new(IconName::AiOpenAi, Text::new("text color").text_color(gpui::red()))

                    Label::new(
                        Icon::new(IconName::Ai).color(gpui::blue()),
                        Text::new("complex")
                            .text_color(gpui::red())
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                    )
                    .line_height(rems(1.8))
                }
                .gap_2()
            }

            Section! {
                "Maksed Label";
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
                .gap_2()
            }
        }
        .size_full()
        .bg(rgb(0xffffff))
        .gap_1()
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
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
