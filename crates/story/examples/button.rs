use gpui::{px, size, Application, Bounds, ClickEvent, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Button, ButtonGroup, Root, Theme};

struct ButtonStory;

impl ButtonStory {
    fn on_click(e: &ClickEvent, _window: &mut Window, _cx: &mut App) {
        println!("Button clicked! {:?}", e);
    }
}

impl Render for ButtonStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Row! {
                Button::new("id").text("Default")
                Button::new("id").text("Primary").primary()
                Button::new("id").text("Secondary").secondary()
                Button::new("id").text("Info").info()
                Button::new("id").text("Success").success()
                Button::new("id").text("Warning").warning()
                Button::new("id").text("Danger").danger()
            }
            .gap_2()

            Row! {
                Button::new("id").text("XSmall").size(Size::XSmall)
                Button::new("id").text("Small").size(Size::Small)
                Button::new("id").text("Medium")
                Button::new("id").text("Large").size(Size::Large)
            }
            .gap_2()

            ButtonGroup::new().children(vec!["One","Two","Three"]).on_click(|v,_,_| println!("Button clicked!{}",v))

            Button::new("id")
                .text("Click me")
                .on_click(Self::on_click)
        }
        .p_4()
        .gap_1()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ButtonStory {}),
        )
        .unwrap();
    });
}
