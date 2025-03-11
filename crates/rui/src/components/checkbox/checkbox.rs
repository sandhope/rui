use crate::{prelude::*, Icon, IconName, IconSize, Text};
use gpui::{px, AnyView};

/// # Checkbox
///
/// Checkboxes are used for multiple choices, not for mutually exclusive choices.
/// Each checkbox works independently from other checkboxes in the list,
/// therefore checking an additional box does not affect any other selections.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    base: Div,
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
            base: div().flex().flex_row().items_center(),
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

impl Styled for Checkbox {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
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

        let bg_color = match self.state {
            ToggleState::Unselected => cx.theme().colors.bg,
            _ => cx.theme().colors.primary,
        };

        let border_color = match self.state {
            ToggleState::Unselected => cx.theme().colors.border,
            _ => cx.theme().colors.primary,
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
                    .map(|this| match self.disabled {
                        true => this.border_color(border_color.opacity(0.5)),
                        false => this
                            .border_color(border_color)
                            .group_hover(group_id.clone(), |this| {
                                this.border_color(cx.theme().colors.element_hover)
                            }),
                    })
                    .children(icon),
            );

        self.base
            .id(self.id)
            .gap(rems_from_px(6.0))
            .cursor_pointer()
            .when(self.disabled, |this| this.cursor_not_allowed().opacity(0.5))
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
