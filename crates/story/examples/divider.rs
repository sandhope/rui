use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Assets, Divider, DividerTitle, Root, Text};

struct DividerStory;

impl Render for DividerStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Section! {
                "Horizontal";
                Divider::new().mb_3()
                Divider::new().dashed().mb_3()
                Divider::new().dashed().text("divider").margin_bottom(3.)
                Divider::new().text("divider").margin_bottom(3.)
                DividerTitle::new("divider".to_uppercase()).margin_bottom(3.)
            }

            Section! {
                "Vertical";
                Row!{
                    Text::new("text")
                    Divider::vertical().dashed()
                    Divider::vertical().dashed().text("divider")
                    Text::new("text")
                }
                .gap_1()
                .h_32()
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
            |_, cx| cx.new(|_| DividerStory {}),
        )
        .unwrap();
    });
}
