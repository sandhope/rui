use crate::prelude::*;

const MASKED: &'static str = "•";

#[derive(IntoElement)]
pub struct Text {
    base: Div,
    text: SharedString,
    color: Option<Hsla>,
    marked: bool,
}

impl Text {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            text: text.into(),
            color: None,
            marked: false,
        }
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.marked = masked;
        self
    }

    pub fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Styled for Text {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = if self.marked {
            SharedString::from(MASKED.repeat(self.text.chars().count()))
        } else {
            self.text
        };

        let color = self.color.unwrap_or_else(|| cx.theme().colors.text);

        self.base.text_color(color).child(text)
    }
}

impl From<&'static str> for Text {
    fn from(text: &'static str) -> Self {
        Self::new(text)
    }
}

impl From<SharedString> for Text {
    fn from(text: SharedString) -> Self {
        Self::new(text)
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}
