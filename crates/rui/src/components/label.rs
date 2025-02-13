use crate::{prelude::*, Icon, Text};

#[derive(IntoElement)]
pub struct Label {
    base: Div,
    icon: Icon,
    text: Text,
    icon_right: bool,
}

impl Label {
    pub fn new(icon: impl Into<Icon>, text: impl Into<Text>) -> Self {
        Self {
            base: div().flex().flex_row().items_center().gap_x_2(),
            icon: icon.into(),
            text: text.into(),
            icon_right: false,
        }
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.text = self.text.masked(masked);
        self
    }

    pub fn icon_right(mut self) -> Self {
        self.icon_right = true;
        self
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.map(|this| {
            if self.icon_right {
                this.child(self.icon).child(self.text)
            } else {
                this.child(self.text).child(self.icon)
            }
        })
    }
}
