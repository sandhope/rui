use crate::{prelude::*, Size, Text};
use gpui::{px, CursorStyle};

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
        let base_color = cx.theme().colors.bg;

        let bg_color = match (self.checked, self.disabled) {
            (true, true) => cx.theme().colors.switch_checked_bg.opacity(0.7),
            (true, false) => cx.theme().colors.switch_checked_bg,
            (false, true) => cx.theme().colors.switch_unchecked_bg.opacity(0.4),
            (false, false) => cx.theme().colors.switch_unchecked_bg,
        };
        let bg_hover_color = if self.checked {
            cx.theme().colors.switch_checked_hover_bg
        } else {
            cx.theme().colors.switch_unchecked_hover_bg
        };
        let thumb_color = base_color.opacity(0.95);
        let thumb_hover_color = base_color.opacity(1.0);

        let group_id = format!("switch_group_{:?}", self.id);

        let (width, height, thumb_size) = match self.size {
            Size::XSmall => (rems_from_px(20.), rems_from_px(14.), rems_from_px(10.)),
            Size::Small => (rems_from_px(26.), rems_from_px(16.), rems_from_px(12.)),
            Size::Medium => (rems_from_px(32.), rems_from_px(20.), rems_from_px(14.)),
            Size::Large => (rems_from_px(40.), rems_from_px(24.), rems_from_px(18.)),
            _ => (rems_from_px(32.), rems_from_px(20.), rems_from_px(14.)),
        };

        let switch = h_flex().w(width).h(height).group(group_id.clone()).child(
            h_flex()
                .when(self.checked, |on| on.justify_end())
                .when(!self.checked, |off| off.justify_start())
                .items_center()
                .size_full()
                .rounded_full()
                .px(px(2.))
                .bg(bg_color)
                .when(self.disabled, |this| {
                    this.cursor(CursorStyle::OperationNotAllowed)
                })
                .when(!self.disabled, |this| {
                    this.group_hover(group_id.clone(), |el| el.bg(bg_hover_color))
                })
                .child(
                    div()
                        .size(thumb_size)
                        .rounded_full()
                        .bg(thumb_color)
                        .when(!self.disabled, |this| {
                            this.group_hover(group_id.clone(), |el| el.bg(thumb_hover_color))
                        }),
                ),
        );

        h_flex()
            .id(self.id)
            .gap(rems_from_px(6.))
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
