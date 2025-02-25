use crate::{prelude::*, Direction, Icon, IconName, IconSize, Text};
use gpui::{px, AnyView, CursorStyle};
use std::cell::RefCell;
use std::rc::Rc;

/// # Checkbox
///
/// Checkboxes are used for multiple choices, not for mutually exclusive choices.
/// Each checkbox works independently from other checkboxes in the list,
/// therefore checking an additional box does not affect any other selections.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    state: ToggleState,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    text: Option<Text>,
}

impl Checkbox {
    /// Creates a new [`Checkbox`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state: ToggleState::default(),
            disabled: false,
            on_click: None,
            tooltip: None,
            text: None,
        }
    }

    pub fn checked(mut self, checked: impl Into<ToggleState>) -> Self {
        self.state = checked.into();
        self
    }

    /// Sets the disabled state of the [`Checkbox`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Binds a handler to the [`Checkbox`] that will be called when clicked.
    pub fn on_click(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Sets the tooltip for the checkbox.
    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    /// Set the text for the checkbox.
    pub fn text(mut self, text: impl Into<Text>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let thumb_color = cx.theme().colors.bg;
        // let thumb_color = gpui::white();
        let group_id = format!("checkbox_group_{:?}", self.id);
        let icon = match self.state {
            ToggleState::Selected => Some(
                Icon::new(IconName::Check)
                    .size(IconSize::Small)
                    .color(thumb_color),
            ),
            ToggleState::Indeterminate => Some(
                Icon::new(IconName::Dash)
                    .size(IconSize::Small)
                    .color(thumb_color),
            ),
            ToggleState::Unselected => None,
        };

        let bg_color = match (self.disabled, !self.state.unselected()) {
            (true, true) => cx.theme().colors.primary_disabled,
            (false, true) => cx.theme().colors.primary,
            (_, false) => cx.theme().colors.bg,
        };

        let border_color = match (self.disabled, !self.state.unselected()) {
            (true, true) => cx.theme().colors.primary_disabled,
            (false, true) => cx.theme().colors.primary,
            (true, false) => cx.theme().colors.border.opacity(0.6),
            (false, false) => cx.theme().colors.border,
        };

        let checkbox = h_flex()
            .id(self.id.clone())
            .justify_center()
            .items_center()
            .size(rems_from_px(20.0))
            .group(group_id.clone())
            .child(
                div()
                    .flex()
                    .flex_none()
                    .justify_center()
                    .items_center()
                    .m(px(4.0))
                    .size(rems_from_px(16.0))
                    .rounded_sm()
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .when(self.disabled, |this| {
                        this.cursor(CursorStyle::OperationNotAllowed)
                    })
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| {
                            el.border_color(cx.theme().colors.element_hover)
                        })
                    })
                    .children(icon),
            );

        h_flex()
            .id(self.id)
            .gap(rems_from_px(6.0))
            .cursor_pointer()
            .child(checkbox)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        on_click(&!self.state.selected(), window, cx)
                    })
                },
            )
            .when_some(self.text, |this, text| this.child(text))
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |window, cx| tooltip(window, cx))
            })
    }
}

impl From<&'static str> for Checkbox {
    fn from(text: &'static str) -> Self {
        Self::new(text).text(Text::new(text))
    }
}

impl From<SharedString> for Checkbox {
    fn from(text: SharedString) -> Self {
        Self::new(text.clone()).text(Text::new(text))
    }
}

impl From<String> for Checkbox {
    fn from(text: String) -> Self {
        Self::new(SharedString::from(text.clone())).text(Text::new(text))
    }
}

/// A Checkbox group element.
#[derive(IntoElement)]
pub struct CheckboxGroup {
    checkboxes: Vec<Checkbox>,
    direction: Direction,
    selected_indexes: Vec<usize>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&Vec<usize>, &mut Window, &mut App) + 'static>>,
}

impl CheckboxGroup {
    pub fn new() -> Self {
        Self {
            on_change: None,
            direction: Direction::Horizontal,
            selected_indexes: vec![],
            disabled: false,
            checkboxes: vec![],
        }
    }

    /// Set the direction of the Checkbox group. Default is `Direction::Horizontal`.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the direction of the Checkbox group to vertical.
    /// This is a convenience method for setting the direction to vertical without passing an argument.
    pub fn direction_vertical(mut self) -> Self {
        self.direction = Direction::Vertical;
        self
    }

    /// Sets the direction of the Checkbox group to horizontal.
    /// This is a convenience method for setting the direction to horizontal without passing an argument.
    pub fn direction_horizontal(mut self) -> Self {
        self.direction = Direction::Horizontal;
        self
    }

    /// Listen to the change event.
    pub fn on_change(
        mut self,
        handler: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Set the selected index.
    pub fn selected_indexes(mut self, indexes: Vec<usize>) -> Self {
        self.selected_indexes = indexes;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Add a child Checkbox element.
    pub fn child(mut self, child: impl Into<Checkbox>) -> Self {
        self.checkboxes.push(child.into());
        self
    }

    /// Add multiple child Checkbox elements.
    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Checkbox>>) -> Self {
        self.checkboxes.extend(children.into_iter().map(Into::into));
        self
    }
}

impl RenderOnce for CheckboxGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_change = self.on_change;
        let disabled = self.disabled;
        let selected_indexes = Rc::new(RefCell::new(self.selected_indexes));

        let base = if self.direction == Direction::Vertical {
            v_flex()
        } else {
            h_flex().flex_wrap()
        };

        base.gap_3().children(
            self.checkboxes
                .into_iter()
                .enumerate()
                .map(|(index, checkbox)| {
                    let selected_indexes = Rc::clone(&selected_indexes);
                    let checked = selected_indexes.borrow().contains(&index);

                    checkbox.disabled(disabled).checked(checked).when_some(
                        on_change.clone(),
                        move |this, on_change| {
                            this.on_click(move |_, window, cx| {
                                let mut selected_indexes = selected_indexes.borrow_mut();
                                if let Some(i) = selected_indexes.iter().position(|&i| i == index) {
                                    selected_indexes.remove(i);
                                } else {
                                    selected_indexes.push(index);
                                }

                                on_change(&selected_indexes, window, cx);
                            })
                        },
                    )
                }),
        )
    }
}
