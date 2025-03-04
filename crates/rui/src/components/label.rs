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

    pub fn icon_right(mut self) -> Self {
        self.icon_right = true;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        let color = color.into();
        self.icon = self.icon.color(color.clone());
        self.text = self.text.text_color(color);
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
        //let text = div().flex_none().line_height(relative(1.)).child(self.text);
        self.base
            // .line_height(relative(1.))
            .map(|this| {
                if self.icon_right {
                    this.child(self.text).child(self.icon)
                } else {
                    this.child(self.icon).child(self.text)
                }
            })
    }
}
