#![allow(missing_docs)]

use std::rc::Rc;

use crate::prelude::*;
use crate::Direction;
use crate::Text;

#[derive(IntoElement)]
pub struct Radio {
    id: ElementId,
    text: Option<Text>,
    checked: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl Radio {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            text: None,
            checked: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn text(mut self, text: impl Into<Text>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, on_click: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }
}

impl RenderOnce for Radio {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let inner_diameter = rems_from_px(6.);
        let outer_diameter = rems_from_px(16.);
        let border_width = rems_from_px(1.);

        h_flex()
            .id(self.id)
            .gap_x_2()
            .group("")
            //.line_height(relative(1.))
            .child(
                div()
                    .size(outer_diameter)
                    .rounded(outer_diameter / 2.)
                    .border_color(cx.theme().colors.border)
                    .border(border_width)
                    // .when(self.disabled, |this| this.opacity(0.5))
                    // .when(!self.disabled, |this| {
                    //     this.group_hover("", |this| this.border_color(cx.theme().element_hover))
                    // })
                    .map(|this| {
                        if self.disabled {
                            this.opacity(0.5)
                        } else {
                            this.group_hover("", |this| {
                                this.border_color(cx.theme().colors.element_hover)
                            })
                        }
                    })
                    .when(self.checked, |this| {
                        this.child(
                            div()
                                .m((outer_diameter - inner_diameter) / 2. - border_width)
                                .size(inner_diameter)
                                .rounded(inner_diameter / 2.)
                                .bg(cx.theme().colors.icon_accent),
                        )
                    }),
            )
            .when_some(self.text, |this, text| this.child(text))
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_event, window, cx| {
                        on_click(&!self.checked, window, cx);
                    })
                },
            )
    }
}

impl From<&'static str> for Radio {
    fn from(text: &'static str) -> Self {
        Self::new(text).text(Text::new(text))
    }
}

impl From<SharedString> for Radio {
    fn from(text: SharedString) -> Self {
        Self::new(text.clone()).text(Text::new(text))
    }
}

impl From<String> for Radio {
    fn from(text: String) -> Self {
        Self::new(SharedString::from(text.clone())).text(Text::new(text))
    }
}

/// A Radio group element.
#[derive(IntoElement)]
pub struct RadioGroup {
    radios: Vec<Radio>,
    direction: Direction,
    selected_index: Option<usize>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl RadioGroup {
    pub fn new() -> Self {
        Self {
            on_change: None,
            direction: Direction::Horizontal,
            selected_index: None,
            disabled: false,
            radios: vec![],
        }
    }

    /// Set the direction of the Radio group. Default is `Direction::Horizontal`.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the direction of the Radio group to vertical.
    /// This is a convenience method for setting the direction to vertical without passing an argument.
    pub fn direction_vertical(mut self) -> Self {
        self.direction = Direction::Vertical;
        self
    }

    /// Sets the direction of the Radio group to horizontal.
    /// This is a convenience method for setting the direction to horizontal without passing an argument.
    pub fn direction_horizontal(mut self) -> Self {
        self.direction = Direction::Horizontal;
        self
    }

    /// Listen to the change event.
    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Set the selected index.
    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Add a child Radio element.
    pub fn child(mut self, child: impl Into<Radio>) -> Self {
        self.radios.push(child.into());
        self
    }

    /// Add multiple child Radio elements.
    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Radio>>) -> Self {
        self.radios.extend(children.into_iter().map(Into::into));
        self
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_change = self.on_change;
        let disabled = self.disabled;
        let selected_index = self.selected_index;

        let base = if self.direction == Direction::Vertical {
            v_flex()
        } else {
            h_flex().flex_wrap()
        };

        div().flex().child(
            base.gap_3()
                .children(self.radios.into_iter().enumerate().map(|(index, radio)| {
                    let checked = selected_index == Some(index);

                    radio.disabled(disabled).checked(checked).when_some(
                        on_change.clone(),
                        |this, on_change| {
                            this.on_click(move |_, window, cx| {
                                on_change(&index, window, cx);
                            })
                        },
                    )
                })),
        )
    }
}
