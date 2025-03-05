use gpui::{px, size, Application, Bounds, ClickEvent, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Button, ButtonGroup, ButtonVariant, Color, Root, Text, Theme};

struct ButtonStory;

impl ButtonStory {
    fn on_click(e: &ClickEvent, _window: &mut Window, _cx: &mut App) {
        println!("Button clicked! {:?}", e);
    }
}

fn color_row(text: impl Into<SharedString>, color: Hsla) -> Div {
    Row! {
        Text::new(text).w_20()
        Button::new("id").text("Solid").color(color)
        Button::new("id").text("Soft").color(color).soft()
        Button::new("id").text("Outline").color(color).outline()
        Button::new("id").text("Ghost").color(color).ghost()
        Button::new("id").text("Plain").color(color).plain()
    }
    .gap_2()
}

fn variant_row(text: impl Into<SharedString>, variant: ButtonVariant) -> Div {
    Row! {
        Text::new(text).w_20()
        Button::new("id").text("Default").variant(variant)
        Button::new("id").text("Primary").variant(variant).primary()
        Button::new("id").text("Secondary").variant(variant).secondary()
        Button::new("id").text("Success").variant(variant).success()
        Button::new("id").text("Warning").variant(variant).warning()
        Button::new("id").text("Danger").variant(variant).danger()
    }
    .gap_2()
}

impl Render for ButtonStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            variant_row("solid", ButtonVariant::Solid)
            variant_row("soft", ButtonVariant::Soft)
            variant_row("outline", ButtonVariant::Outline)
            variant_row("ghost", ButtonVariant::Ghost)
            variant_row("plain", ButtonVariant::Plain)

            color_row("black", Color::black())
            color_row("red", Color::red())
            color_row("green", Color::green())
            color_row("blue", Color::blue())
            color_row("teal", Color::teal())
            color_row("pink", Color::pink())
            color_row("purple", Color::purple())
            color_row("cyan", Color::cyan())
            color_row("orange", Color::orange())
            color_row("yellow", Color::yellow())

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
                .soft()
                .primary()
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
