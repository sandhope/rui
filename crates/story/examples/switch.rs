use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Root, Section, Switch, SwitchWithText, Text};

struct SwitchStory {
    state: ToggleState,
    state_bool: bool,
    disabled: bool,
}

impl Render for SwitchStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Section! {
                "switch";
                Switch::new("row", self.state)
                    .text("change disabled")
                    .on_click(cx.listener(|this, s: &ToggleState, _window, _cx| {
                        println!("{}", s.selected());
                        this.state = *s;
                        this.disabled = !this.disabled;
                    }))

                Switch::new("row", self.state_bool.into())
                    .text(if self.disabled { "disabled" } else { "enabled" })
                    .disabled(self.disabled)
                    .on_click(cx.listener(|this, s: &ToggleState, _window, _cx| {
                        println!("{}", s.selected());
                        this.state_bool = s.selected();
                    }))

                SwitchWithText::new(
                    "SwitchWithText",
                    Text::new("xx").text_color(gpui::blue()).text_xs(),
                    self.state_bool,
                    cx.listener(|this, s: &ToggleState, _window, _cx| {
                        println!("{}", s.selected());
                        this.state_bool = s.selected();
                    }))
                    .disabled(self.disabled)
            }
            .gap_1()
        }
        .px_4()
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
            |_, cx| {
                cx.new(|_| SwitchStory {
                    state: ToggleState::Unselected,
                    state_bool: false,
                    disabled: true,
                })
            },
        )
        .unwrap();
    });
}
