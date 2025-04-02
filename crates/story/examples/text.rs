use gpui::FontWeight;
use rui::{prelude::*, Button, DividerTitle, IconName, Root, Row, Section, Text};

struct TextStory {
    masked: bool,
}

impl Render for TextStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Section! {
                "Text";
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


                div().w(px(200.)).child(
                    Text::new("Label should support text wrap in default, if the text is too long, it should wrap to the next line.")
                        .line_height(rems(1.8)),
                )

                DividerTitle::new("Styles")

                Row! {
                    Col! {
                        Text::new("Default")
                        Text::new("Normal Text")
                    }
                    Col! {
                        Text::new("Bold")
                        Text::new("Bold").font_weight(gpui::FontWeight::BOLD)
                    }
                    Col! {
                        Text::new("Italic")
                        Text::new("Italic").italic()
                    }
                    Col! {
                        Text::new("Strikethrough")
                        Text::new("Strikethrough").strikethrough()
                    }
                    Col! {
                        Text::new("Underline")
                        Text::new("Underline").underline()
                    }
                }
                .gap_4()

                DividerTitle::new("Special Cases")
                Row! {
                    Col! {
                        Text::new("Single Line")
                        Text::new("Line 1\nLine 2\nLine 3").single_line()
                    }
                    Col! {
                        Text::new("Text Ellipsis")
                        Text::new("This is a very long file name that should be truncated: very_long_file_name_with_many_words.rs").max_w_24().truncate()
                    }
                }
                .gap_4()

                DividerTitle::new("Masked")
                Row! {
                    Text::new("123,456,789 $").text_2xl().masked(self.masked).w_40()
                    Button::new("btn-mask")
                        .icon(if self.masked {
                            IconName::EyeOff
                        } else {
                            IconName::Eye
                        })
                        .ghost()
                        .on_click(cx.listener(|this, _, _, _| {
                            this.masked = !this.masked;
                        }))
                }
                .gap_4()
            }
        }
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
            |_, cx| {
                let view = cx.new(|_| TextStory { masked: false });
                cx.new(|cx| Root::new(cx, view.into()))
            },
        )
        .unwrap();
    });
}
