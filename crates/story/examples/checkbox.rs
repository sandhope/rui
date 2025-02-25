use gpui::{px, size, Application, Bounds, Context, WindowBounds, WindowOptions};

use rui::{prelude::*, Assets, Checkbox, CheckboxGroup, Root, Section, Text, ToggleState};

struct CheckboxStory {
    state: bool,
    second_state: bool,
    disabled: bool,
    selected_indexes: Vec<usize>,
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
                CheckboxGroup::new()
                    .selected_indexes(self.selected_indexes.clone())
                    .children(["One", "Two", "Three"])
                    .on_change(cx.listener(|this, selected_indexes: &Vec<usize>, _, _| {
                        println!("{:?}",*selected_indexes);
                        this.selected_indexes = selected_indexes.clone();
                    }))
            }

            Section! {
                "Checkbox Group Vertical";
                CheckboxGroup::new()
                    .direction_vertical()
                    .disabled(self.disabled)
                    .selected_indexes(self.selected_indexes.clone())
                    .child(Checkbox::new("one1").text("one1"))
                    .child(Checkbox::new("one2").text("one2"))
                    .child(Checkbox::new("one3").text(Text::new("one3")))
                    .on_change(cx.listener(|this, selected_indexes: &Vec<usize>, _, _| {
                        println!("{:?}",*selected_indexes);
                        this.selected_indexes = selected_indexes.clone();
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
                    selected_indexes: Vec::new(),
                })
            },
        )
        .unwrap();
    });
}
