use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Assets, Checkbox, Root, Section, Text, ToggleState};

struct CheckboxStory {
    state: bool,
    second_state: bool,
    disabled: bool,
}

impl Render for CheckboxStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Section! {
                "switch";
                Checkbox::new("id")
                    .checked(self.state)
                    .text("change disabled")
                    .on_click(cx.listener(|this, v, _window, _cx| {
                        this.state = *v;
                        this.disabled = !this.disabled;
                    }))

                Checkbox::new("id")
                    .checked(self.second_state)
                    .text(if self.disabled { "disabled" } else { "enabled" })
                    .disabled(self.disabled)
                    .on_click(cx.listener(|this, v, _window, _cx| {
                        this.second_state = *v;
                    }))

                Checkbox::new("id").checked(true)
                Checkbox::new("id").checked(ToggleState::Indeterminate)

                Checkbox::new("id")
                    .checked(cx.theme().appearance.is_light())
                    .text(Text::new("appearance").text_color(gpui::blue()))
                    .on_click(cx.listener(|_, _v, window, cx| {
                        cx.theme_mut().toggle_builtin_appearance(window);
                    }))
            }
            .gap_1()
        }
        .px_4()
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
            |_, cx| {
                cx.new(|_| CheckboxStory {
                    state: false,
                    second_state: false,
                    disabled: true,
                })
            },
        )
        .unwrap();
    });
}
