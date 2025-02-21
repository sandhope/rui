use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{
    prelude::*, Assets, IconName, Label, Radio, RadioGroup, Root, Row, Section, Switch, Text,
    ThemeMode,
};

struct RadioStory {
    enabled: bool,
    selected_index: Option<usize>,
}

impl Render for RadioStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Row! {
                Text::new("Appearance: ").w_1_4()
                Switch::new("appearance", cx.theme().appearance.is_light())
                .text(cx.theme().appearance.to_string())
                .on_click(cx.listener(|_, _, window, cx| {
                    cx.theme_mut().toggle_builtin_appearance(window);
                    println!("{:?}", cx.theme().appearance);
                }))
            }
            .w_full()

            Row! {
                Text::new("label with theme").w(relative(0.25))
                Label::new(IconName::AiZed, "label")
            }
            .w_full()

            Section! {
                "Radio Group";
                RadioGroup::new()
                    .children(["One", "Two", "Three"])
                    .selected_index(self.selected_index)
                    .on_change(cx.listener(|this, selected_index: &usize, _, _| {
                        this.selected_index = Some(*selected_index);
                    }))
            }
            .gap_4()

            Row! {
                Text::new("disable RadioGroup").w_1_4()
                Switch::new("radio", self.enabled)
                .text(if self.enabled {
                    "enabled"
                } else {
                    "disabled"
                })
                .on_click(cx.listener(|this, s: &ToggleState, _window, _app| {
                    // this.enabled = !this.enabled
                    this.enabled = s.selected();
                }))
            }
            .w_full()

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
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        Theme::init(cx, None, Some(ThemeMode::Dark));
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
