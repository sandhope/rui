use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Button, Headline, HeadlineSize, Root, Theme};

struct HeadlineStory;

impl Render for HeadlineStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Section!{
                "Headline";
                Headline::new("XLarge")
                    .size(HeadlineSize::XLarge)
                Headline::new("Large")
                    .size(HeadlineSize::Large)
                Headline::new("Medium")
                Headline::new("Small")
                    .size(HeadlineSize::Small)
                Headline::new("XSmall")
                    .size(HeadlineSize::XSmall)
            }

            Button::new("appearance")
                .text(cx.theme().appearance.to_string())
                .on_click(cx.listener(|_, _, window, cx| {
                    cx.theme_mut().toggle_builtin_appearance(window);
                    println!("{:?}", cx.theme().appearance);
                }))
        }
        .p_4()
        .gap_1()
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
            |_, cx| cx.new(|_| HeadlineStory {}),
        )
        .unwrap();
    });
}
