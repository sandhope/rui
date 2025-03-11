use gpui::{px, rems, size, Application, Bounds, Context, FontWeight, WindowBounds, WindowOptions};

use rui::{prelude::*, Assets, Col, Icon, IconName, IconSize, Label, Root, Row, Section, Text};

struct LabelStory;

impl Render for LabelStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Section! {
                "Label row";
                Row!{
                    Label::new(IconName::Eye, "simple")
                    Label::new(Icon::new(IconName::Bell), "icon new")
                    Label::new(Icon::new(IconName::Ai).color(gpui::blue()), "icon color")
                    Label::new(Icon::new(IconName::Ai).color(gpui::blue()), "icon right").icon_right()
                    Label::new(IconName::AiOpenAi, Text::new("text new"))
                    Label::new(IconName::AiOpenAi, Text::new("text color").text_color(gpui::red()))
                    Label::new(IconName::AiOpenAi, "label color").color(gpui::blue())
                    Label::new(
                        Icon::new(IconName::Ai).size(IconSize::Large).color(gpui::blue()),
                        Text::new("complex")
                            .text_color(gpui::red())
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                    )
                    .line_height(rems(1.8))
                }
                .gap_4()
            }

            Section! {
                "Label col";
                Col!{
                    Label::new(IconName::Eye, "simple")
                    Label::new(Icon::new(IconName::Bell), "icon new")
                    Label::new(Icon::new(IconName::Ai).color(gpui::blue()), "icon color")
                    Label::new(Icon::new(IconName::Ai).color(gpui::blue()), "icon right").icon_right()
                    Label::new(IconName::AiOpenAi, Text::new("text new"))
                    Label::new(IconName::AiOpenAi, Text::new("text color").text_color(gpui::red()))
                    Label::new(IconName::AiOpenAi, "label color").color(gpui::blue())
                    Label::new(
                        Icon::new(IconName::Ai).size(IconSize::Large).color(gpui::blue()),
                        Text::new("complex")
                            .text_color(gpui::red())
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                    )
                    .line_height(rems(1.8))
                }
                .gap_3()
            }
        }
        .font_family(".SystemUIFont")
        .px_4()
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        cx.activate(true);
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| LabelStory {}),
        )
        .unwrap();
    });
}
