use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Radio, RadioGroup, Root, Section, Text};

struct RadioStory {
    enabled: bool,
    selected_index: Option<usize>,
}

impl Render for RadioStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Radio::new("appearance, to do: change to switch")
            .text(cx.theme().appearance.to_string())
            .checked(cx.theme().appearance.is_light())
            .on_click(cx.listener(|_, _, window, cx| {
                cx.theme_mut().toggle_builtin_appearance(window);
                println!("{:?}", cx.theme().appearance);
            }))

            Radio::new("radio")
            .text(if self.enabled {
                "enabled"
            } else {
                "disabled"
            })
            .checked(self.enabled)
            .on_click(cx.listener(|this, val, _window, _app| {
                // this.checked = !this.checked
                this.enabled = *val
            }))

            Section! {
                "Radio Group";
                RadioGroup::new()
                    .children(["One", "Two", "Three"])
                    .selected_index(self.selected_index)
                    .on_change(cx.listener(|this, selected_index: &usize, _, _| {
                        this.selected_index = Some(*selected_index);
                    }))
            }

            Section! {
                "Radio Group Vertical";
                RadioGroup::new()
                    .direction_vertical()
                    .disabled(!self.enabled)
                    .child(Radio::new("one1").text("one1"))
                    .child(Radio::new("one2").text("one2"))
                    .child(Radio::new("one3").text(Text::new("one3")))
                    .selected_index(self.selected_index)
                    .on_change(cx.listener(|this, selected_index: &usize, _, _| {
                        this.selected_index = Some(*selected_index);
                    }))
            }
        }
        .justify_start()
        .items_start()
        .text_lg()
        .p_8()
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
                cx.new(|_| RadioStory {
                    enabled: false,
                    selected_index: None,
                })
            },
        )
        .unwrap();
    });
}
