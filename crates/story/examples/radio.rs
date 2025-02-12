use gpui::{px, rgb, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Col, Radio, RadioGroup, Text};

struct RadioStory {
    checked: bool,
    selected_index: Option<usize>,
}

impl Render for RadioStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Col! {
            Radio::new("radio")
            .text(Text::new("radio"))
            .checked(self.checked)
            .on_click(cx.listener(|this, val, _window, _app| {
                // this.checked = !this.checked
                this.checked = *val
            }))

            RadioGroup::new()
                .children(["One", "Two", "Three"])
                .selected_index(self.selected_index)
                .on_change(cx.listener(|this, selected_index: &usize, _, _| {
                    this.selected_index = Some(*selected_index);
                }))

            RadioGroup::new()
                .direction(gpui::Axis::Vertical)
                .disabled(true)
                .child(Radio::new("one1").text(Text::new("one1")))
                .child(Radio::new("one2").text(Text::new("one2")))
                .child(Radio::new("one3").text(Text::new("one3")))
                .selected_index(self.selected_index)
                .on_change(cx.listener(|this, selected_index: &usize, _, _| {
                    this.selected_index = Some(*selected_index);
                }))
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
                    checked: false,
                    selected_index: None,
                })
            },
        )
        .unwrap();
    });
}
