use gpui::{size, Application, Bounds, WindowBounds, WindowOptions};
use rui::{prelude::*, Color, Icon, IconName, Link, Root, Row, Switch, Text};

struct LinkStory;

impl Render for LinkStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Row! {
                Text::new("Appearance: ").w_1_4()
                Switch::new("appearance")
                    .checked(cx.theme().appearance.is_light())
                    .text(cx.theme().appearance.to_string())
                    .on_click(cx.listener(|_, _, window, cx| {
                        cx.theme_mut().toggle_builtin_appearance(window);
                        println!("{:?}", cx.theme().appearance);
                    }))
            }

            Link::new("id").href("https://github.com").child(IconName::Github)
            Link::new("id").href("https://example.com").child("Example")
            Link::new("id").href("https://github.com").child("GitHub")
            Link::new("id").href("https://github.com").child(IconName::Github).child("GitHub")
            Link::new("id").child(Text::new("GitBranch").color(Color::red())).child(IconName::GitBranch)
            Link::new("id").child(Icon::new(IconName::GitBranch).color(Color::teal())).child("GitBranch")
            Link::new("id").child("GitBranch").child(IconName::GitBranch)
        }
        .px_4()
        .gap_2()
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| LinkStory {}),
        )
        .unwrap();
    });
}
