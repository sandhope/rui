use gpui::{px, size, Application, Bounds, ClickEvent, Context, WindowBounds, WindowOptions};

use rui::{
    prelude::*, Button, ButtonGroup, ButtonVariant, Color, Icon, IconName, IconSize, Root, Text,
    Theme,
};

struct ButtonStory;

impl ButtonStory {
    fn on_click(_e: &ClickEvent, window: &mut Window, cx: &mut App) {
        cx.theme_mut().toggle_builtin_appearance(window);
    }
}

fn color_row(text: impl Into<SharedString>, color: Hsla) -> Div {
    Row! {
        Text::new(text).w_20()
        Button::new("id").text("Solid").color(color)
        Button::new("id").text("Soft").color(color).surface()
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
        Button::new("id").text("Default").variant(variant).disabled(true)
        Button::new("id").text("Default").variant(variant).primary().disabled(true)
        Button::new("id").text("Secondary").variant(variant).secondary().disabled(true)
        Button::new("id").text("Success").variant(variant).success().disabled(true)
        Button::new("id").text("Warning").variant(variant).warning().disabled(true)
        Button::new("id").text("Default").variant(variant).danger().disabled(true)
    }
    .gap_2()
}

impl Render for ButtonStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            variant_row("Solid", ButtonVariant::Solid)
            variant_row("Surface", ButtonVariant::Surface)
            variant_row("Soft", ButtonVariant::Soft)
            variant_row("Outline", ButtonVariant::Outline)
            variant_row("Ghost", ButtonVariant::Ghost)
            variant_row("Plain", ButtonVariant::Plain)

            color_row("Black", Color::black())
            color_row("Red", Color::red())
            color_row("Green", Color::green())
            color_row("Blue", Color::blue())
            color_row("Teal", Color::teal())
            color_row("Pink", Color::pink())
            color_row("Purple", Color::purple())
            color_row("Cyan", Color::cyan())
            color_row("Orange", Color::orange())
            color_row("Yellow", Color::yellow())

            Row! {
                Button::new("id").text("XSmall").size(Size::XSmall)
                Button::new("id").text("Small").size(Size::Small)
                Button::new("id").text("Medium")
                Button::new("id").text("Large").size(Size::Large)

                Button::new("id").icon(IconName::Mic)
                Button::new("id").icon(IconName::Close).danger()
                Button::new("id").text("icon").icon(IconName::Mic)
                Button::new("id").text("icon size").icon(Icon::new(IconName::Mic).size(IconSize::Large))
                Button::new("id").text("icon right").icon(IconName::Mic).icon_right()
                Button::new("id").text("icon color").icon(Icon::new(IconName::Mic).color(Color::red()))
                Button::new("id")
                    .icon(Icon::new(IconName::Mic).color(Color::blue()))
                    .text(Text::new("custom").color(Color::blue()))
                Button::new("id").text("Loading").loading(true)
                Button::new("id")
                    .text("Loading")
                    .loading_icon(Icon::new(IconName::LoadingHalf).color(Color::pink())).loading(true)
            }
            .gap_2()

            Row! {
                ButtonGroup::new().children(vec!["One","Two","Three"])
                ButtonGroup::new().children(vec!["One","Two","Three"]).soft().secondary()
                ButtonGroup::new().children(vec!["One","Two","Three"]).outline().primary()
                ButtonGroup::new().children(vec!["One","Two","Three"]).ghost().success()
            }
            .gap_2()

            Button::new("id")
                .text("Click me")
                .primary()
                .on_click(Self::on_click)
        }
        .p_4()
        .gap_1()
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
            |_, cx| cx.new(|_| ButtonStory {}),
        )
        .unwrap();
    });
}
