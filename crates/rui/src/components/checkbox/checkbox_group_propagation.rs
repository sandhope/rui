use crate::{prelude::*, Checkbox, Direction};
use std::{cell::Cell, rc::Rc};

/// A Checkbox group element.
#[derive(IntoElement)]
pub struct CheckboxGroup {
    id: ElementId,
    checkboxes: Vec<Checkbox>,
    direction: Direction,
    checked_indexes: Vec<usize>,
    disabled: bool,
    on_change: Option<Box<dyn Fn(&Vec<usize>, &mut Window, &mut App) + 'static>>,
}

impl CheckboxGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checkboxes: vec![],
            direction: Direction::Horizontal,
            checked_indexes: vec![],
            disabled: false,
            on_change: None,
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
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Set the selected index.
    pub fn checked_indexes(mut self, indexes: Vec<usize>) -> Self {
        self.checked_indexes = indexes;
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
        let disabled = self.disabled;
        let state = Rc::new(Cell::new(None));

        let base = if self.direction == Direction::Vertical {
            v_flex()
        } else {
            h_flex().flex_wrap()
        };

        let children = self
            .checkboxes
            .into_iter()
            .enumerate()
            .map(|(index, checkbox)| {
                let checked = self.checked_indexes.contains(&index);
                let state = Rc::clone(&state);

                checkbox
                    .disabled(disabled)
                    .checked(checked)
                    .on_click(move |_, _window, _cx| {
                        state.set(Some(index));
                    })
            });

        base.id(self.id).gap_3().children(children).when_some(
            self.on_change.filter(|_| !self.disabled),
            move |this, on_change| {
                this.on_click(move |_, window, cx| {
                    if let Some(index) = state.get() {
                        let mut checked_indexes = self.checked_indexes.clone();

                        if let Some(i) = checked_indexes.iter().position(|&i| i == index) {
                            checked_indexes.remove(i);
                        } else {
                            checked_indexes.push(index);
                        }

                        on_change(&checked_indexes, window, cx);
                    }
                })
            },
        )
    }
}
