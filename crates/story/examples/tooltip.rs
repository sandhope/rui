use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Button, Checkbox, IconName, Link, Root, Switch, Theme, Tooltip};

struct TooltipStory;

impl Render for TooltipStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Button::new("appearance")
                .text(cx.theme().appearance.to_string())
                .tooltip(Tooltip::text("This is a tooltip!"))
                .on_click(cx.listener(|_, _, window, cx| {
                    cx.theme_mut().toggle_builtin_appearance(window);
                    println!("{:?}", cx.theme().appearance);
                }))
            Row! {
                Button::new("btn-tooltip").text("button tooltip").tooltip(Tooltip::text("This is a Button!"))
            }
            Checkbox::new("checkbox-tooltip")
                .text("tooltip")
                .tooltip(Tooltip::text("This is a Checkbox!"))
            Switch::new("switch-id")
                .text("tooltip")
                .tooltip(Tooltip::text("This is a Switch!"))
            Link::new("link-id")
                .href("https://github.com")
                .child(IconName::Github)
                .tooltip(Tooltip::text("This is a Link!"))
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
            |_, cx| cx.new(|_| TooltipStory {}),
        )
        .unwrap();
    });
}
