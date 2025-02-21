use crate::{prelude::*, Size, Text};
use gpui::px;

/// # Switch
///
/// Switches are used to represent opposite states, such as enabled or disabled.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    text: Option<Text>,
    size: Size,
}

impl Switch {
    /// Creates a new [`Switch`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            on_click: None,
            text: None,
            size: Size::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets the disabled state of the [`Switch`].
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Binds a handler to the [`Switch`] that will be called when clicked.
    pub fn on_click(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Sets the text of the [`Switch`].
    pub fn text(mut self, text: impl Into<Text>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_on = self.checked;
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

        let (width, height, thumb_size) = match self.size {
            Size::XSmall => (px(20.), px(14.), px(10.)),
            Size::Small => (px(26.), px(16.), px(12.)),
            Size::Medium => (px(32.), px(20.), px(14.)),
            Size::Large => (px(40.), px(24.), px(18.)),
            _ => (px(32.), px(20.), px(14.)),
        };

        let switch = h_flex().w(width).h(height).group(group_id.clone()).child(
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
                        .size(thumb_size)
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
                    this.on_click(move |_, window, cx| on_click(&!self.checked, window, cx))
                },
            )
            // If Switch set the text size by matching self.size, the text size cannot be set externally
            // .when_some(self.text, |this, text| match self.size {
            //     Size::XSmall => this.child(text.text_xs()),
            //     Size::Small => this.child(text.text_sm()),
            //     Size::Medium => this.child(text.text_base()),
            //     Size::Large => this.child(text.text_lg()),
            //     _ => this.child(text.text_sm()),
            // })
            .when_some(self.text, |this, text| this.child(text))
    }
}
