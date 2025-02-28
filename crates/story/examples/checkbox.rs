use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Checkbox, CheckboxGroup, Divider, Text, ToggleState};

struct CheckboxStory {
    state: bool,
    second_state: bool,
    disabled: bool,
    checked_indexes: Vec<usize>,
    all_state: ToggleState,
}

impl CheckboxStory {
    fn change_handler(&mut self, value: &Vec<usize>) {
        println!("{:?}", *value);
        self.checked_indexes = value.clone();
        match self.checked_indexes.len() {
            3 => self.all_state = ToggleState::Selected,
            0 => self.all_state = ToggleState::Unselected,
            _ => self.all_state = ToggleState::Indeterminate,
        }
    }
}

impl Render for CheckboxStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Root! {
            Section! {
                "Checkbox";
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

                Checkbox::new("id").checked(true).text("checked")
                Checkbox::new("id").checked(ToggleState::Indeterminate).text("Indeterminate")

                Checkbox::new("id")
                    .checked(cx.theme().appearance.is_light())
                    .text(Text::new("appearance").text_color(gpui::blue()))
                    .on_click(cx.listener(|_, _v, window, cx| {
                        cx.theme_mut().toggle_builtin_appearance(window);
                    }))
            }
            .gap_1()


            Section! {
                "Checkbox Group";

                Checkbox::new("id")
                    .checked(self.all_state)
                    .text("Checkall")
                    .on_click(cx.listener(|this, _v, _window, _cx| {
                    if this.all_state.selected() {
                        this.checked_indexes.clear();
                        this.all_state = false.into();
                    } else {
                        this.checked_indexes = vec![0, 1, 2];
                        this.all_state = true.into();
                    }
                }))

                Divider::new().py_2()

                CheckboxGroup::new()
                    .checked_indexes(self.checked_indexes.clone())
                    .children(["One", "Two", "Three"])
                    .on_change(cx.listener(|this, checked_indexes: &Vec<usize>, _, _| {
                        this.change_handler(checked_indexes)
                    }))
            }

            Section! {
                "Checkbox Group Vertical";
                CheckboxGroup::new()
                    .direction_vertical()
                    .disabled(self.disabled)
                    .checked_indexes(self.checked_indexes.clone())
                    .child(Checkbox::new("one1").text("one1"))
                    .child(Checkbox::new("one2").text("one2"))
                    .child(Checkbox::new("one3").text(Text::new("one3")))
                    .on_change(cx.listener(|this, checked_indexes: &Vec<usize>, _, _| {
                        this.change_handler(checked_indexes)
                    }))
            }
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
                    checked_indexes: Vec::new(),
                    all_state: ToggleState::Unselected,
                })
            },
        )
        .unwrap();
    });
}
