use crate::{prelude::*, Text};
use gpui::px;
use std::sync::Arc;

/// # Switch
///
/// Switches are used to represent opposite states, such as enabled or disabled.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    toggle_state: ToggleState,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>>,
    text: Option<SharedString>,
}

impl Switch {
    /// Creates a new [`Switch`].
    pub fn new(id: impl Into<ElementId>, state: ToggleState) -> Self {
        Self {
            id: id.into(),
            toggle_state: state,
            disabled: false,
            on_click: None,
            text: None,
        }
    }

    /// Sets the disabled state of the [`Switch`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Binds a handler to the [`Switch`] that will be called when clicked.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Sets the text of the [`Switch`].
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_on = self.toggle_state == ToggleState::Selected;
        let adjust_ratio = if cx.theme().appearance.is_light() {
            1.5
        } else {
            1.0
        };
        let base_color = cx.theme().colors.bg;

        let bg_color = if is_on {
            cx.theme().colors.switch_bg.blend(base_color.opacity(0.08))
        } else {
            cx.theme().colors.element_background.opacity(0.6)
        };
        let bg_hover_color = if is_on {
            cx.theme().colors.switch_hover_bg
        } else {
            cx.theme().colors.element_background
        };
        let thumb_color = base_color.opacity(0.9);
        let thumb_hover_color = base_color.opacity(1.0);
        let border_color = cx.theme().colors.border_variant;
        // Lighter themes need higher contrast borders
        let border_hover_color = if is_on {
            border_color.blend(base_color.opacity(0.16 * adjust_ratio))
        } else {
            border_color.blend(base_color.opacity(0.05 * adjust_ratio))
        };
        let thumb_opacity = match (is_on, self.disabled) {
            (_, true) => 0.6,
            (true, false) => 1.0,
            (false, false) => 1.0,
        };

        let group_id = format!("switch_group_{:?}", self.id);

        let switch = h_flex()
            .w(px(32.))
            .h(px(20.))
            .group(group_id.clone())
            .child(
                h_flex()
                    .when(is_on, |on| on.justify_end())
                    .when(!is_on, |off| off.justify_start())
                    .items_center()
                    .size_full()
                    .rounded_full()
                    .px(px(1.))
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| {
                            el.border_color(border_hover_color).bg(bg_hover_color)
                        })
                    })
                    .child(
                        div()
                            .size(px(14.))
                            .rounded_full()
                            .bg(thumb_color)
                            .when(!self.disabled, |this| {
                                this.group_hover(group_id.clone(), |el| el.bg(thumb_hover_color))
                            })
                            .opacity(thumb_opacity),
                    ),
            );

        h_flex()
            .id(self.id)
            .gap(px(6.))
            .cursor_pointer()
            .child(switch)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        on_click(&self.toggle_state.inverse(), window, cx)
                    })
                },
            )
            .when_some(self.text, |this, text| {
                this.child(Text::new(text).text_xs())
            })
    }
}

/// A [`Switch`] that has a [`Text`].
#[derive(IntoElement)]
// #[component(scope = "input")]
pub struct SwitchWithText {
    id: ElementId,
    text: Text,
    toggle_state: ToggleState,
    on_click: Arc<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>,
    disabled: bool,
}

impl SwitchWithText {
    /// Creates a switch with an attached text.
    pub fn new(
        id: impl Into<ElementId>,
        text: Text,
        toggle_state: impl Into<ToggleState>,
        on_click: impl Fn(&ToggleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            text,
            toggle_state: toggle_state.into(),
            on_click: Arc::new(on_click),
            disabled: false,
        }
    }

    /// Sets the disabled state of the [`SwitchWithText`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for SwitchWithText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(SharedString::from(format!("{}-container", self.id)))
            .gap(px(6.))
            .child(
                Switch::new(self.id.clone(), self.toggle_state)
                    .disabled(self.disabled)
                    .on_click({
                        let on_click = self.on_click.clone();
                        move |checked, window, cx| {
                            (on_click)(checked, window, cx);
                        }
                    }),
            )
            .child(
                div()
                    .id(SharedString::from(format!("{}-text", self.id)))
                    .child(self.text),
            )
    }
}
