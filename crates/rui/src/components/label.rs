use crate::{prelude::*, Icon, Text};

#[derive(IntoElement)]
pub struct Label {
    base: Div,
    icon: Icon,
    text: Text,
    icon_right: bool,
    color: Option<Hsla>,
}

impl Label {
    pub fn new(icon: impl Into<Icon>, text: impl Into<Text>) -> Self {
        Self {
            base: div().flex().flex_row().items_center().gap_x_2(),
            icon: icon.into(),
            text: text.into(),
            icon_right: false,
            color: None,
        }
    }

    pub fn icon_right(mut self) -> Self {
        self.icon_right = true;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
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
        self.base
            .when_some(self.color, |this, color| this.text_color(color))
            .map(|this| match self.icon_right {
                true => this.child(self.text).child(self.icon),
                false => this.child(self.icon).child(self.text),
            })
    }
}
