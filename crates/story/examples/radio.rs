use gpui::{px, rgb, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Col, Radio, RadioGroup, Section, Text};

struct RadioStory {
    disabled: bool,
    selected_index: Option<usize>,
}

impl Render for RadioStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Radio::new("radio")
            .text("disabled") // or .text(Text::new("disabled"))
            .checked(self.disabled)
            .on_click(cx.listener(|this, val, _window, _app| {
                // this.checked = !this.checked
                this.disabled = *val
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
                    .disabled(self.disabled)
                    .child(Radio::new("one1").text("one1"))
                    .child(Radio::new("one2").text("one2"))
                    .child(Radio::new("one3").text(Text::new("one3")))
                    .selected_index(self.selected_index)
                    .on_change(cx.listener(|this, selected_index: &usize, _, _| {
                        this.selected_index = Some(*selected_index);
                    }))
            }
        }
        .bg(gpui::white())
        .size_full()
        .justify_start()
        .items_start()
        .text_xl()
        .text_color(rgb(0x000000))
        .p_10()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1024.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| RadioStory {
                    disabled: true,
                    selected_index: None,
                })
            },
        )
        .unwrap();
    });
}
