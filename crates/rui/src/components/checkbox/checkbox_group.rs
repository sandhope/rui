use super::Checkbox;
use crate::{prelude::*, Direction};
use std::cell::RefCell;
use std::rc::Rc;

/// A Checkbox group element.
#[derive(IntoElement)]
pub struct CheckboxGroup {
    base: Div,
    checkboxes: Vec<Checkbox>,
    direction: Direction,
    checked_indexes: Vec<usize>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&Vec<usize>, &mut Window, &mut App) + 'static>>,
}

impl CheckboxGroup {
    pub fn new() -> Self {
        Self {
            base: div(),
            on_change: None,
            direction: Direction::Horizontal,
            checked_indexes: vec![],
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
    pub fn vertical(mut self) -> Self {
        self.direction = Direction::Vertical;
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
        let on_change = self.on_change;
        let disabled = self.disabled;
        let checked_indexes = Rc::new(RefCell::new(self.checked_indexes));

        let children = self
            .checkboxes
            .into_iter()
            .enumerate()
            .map(|(index, checkbox)| {
                let checked_indexes = Rc::clone(&checked_indexes);
                let checked = checked_indexes.borrow().contains(&index);

                checkbox.disabled(disabled).checked(checked).when_some(
                    on_change.clone(),
                    move |this, on_change| {
                        this.on_click(move |_, window, cx| {
                            let mut checked_indexes = checked_indexes.borrow_mut();
                            if let Some(i) = checked_indexes.iter().position(|&i| i == index) {
                                checked_indexes.remove(i);
                            } else {
                                checked_indexes.push(index);
                            }

                            on_change(&checked_indexes, window, cx);
                        })
                    },
                )
            });

        self.base
            .map(|this| match self.direction {
                Direction::Vertical => this.v_flex(),
                Direction::Horizontal => this.h_flex().flex_wrap(),
            })
            .gap_3()
            .children(children)
    }
}
