use rui::{prelude::*, Root, Section, Switch, Text};

struct SwitchStory {
    state: bool,
    second_state: bool,
    disabled: bool,
}

impl Render for SwitchStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Section! {
                "switch";
                Switch::new("id")
                    .checked(self.state)
                    .text("change disabled")
                    .on_click(cx.listener(|this, v, _window, _cx| {
                        this.state = *v;
                        this.disabled = !this.disabled;
                    }))

                Switch::new("id")
                    .checked(self.second_state)
                    .text(if self.disabled { "disabled" } else { "enabled" })
                    .disabled(self.disabled)
                    .on_click(cx.listener(|this, v, _window, _cx| {
                        this.second_state = *v;
                    }))

                Switch::new("id")
                    .checked(cx.theme().appearance.is_light())
                    .text(Text::new(cx.theme().appearance.to_string()).color(gpui::blue()))
                    .on_click(cx.listener(|_, _v, window, cx| {
                        cx.theme_mut().toggle_builtin_appearance(window);
                    }))

                // If Switch set the text size by matching self.size, the text size cannot be set externally
                Switch::new("id").text(Text::new("XSmall").text_xs()).size(Size::XSmall)
                Switch::new("id").text(Text::new("Small").text_sm()).size(Size::Small)
                Switch::new("id").text("Medium").size(Size::Medium)
                Switch::new("id").text(Text::new("Large").text_xl()).size(Size::Large)
            }
            .gap_1()
        }
        .px_4()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        Theme::init(cx, None, None);
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let view = cx.new(|_| SwitchStory {
                    state: false,
                    second_state: false,
                    disabled: true,
                });
                cx.new(|cx| Root::new(cx, view.into()))
            },
        )
        .unwrap();
    });
}
