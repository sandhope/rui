use crate::prelude::*;

#[derive(IntoElement)]
pub struct Text {
    base: Div,
    text: SharedString,
    color: Option<Hsla>,
}

impl Text {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            text: text.into(),
            color: None,
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
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
        self.base
            .text_color(self.color.unwrap_or(cx.theme().colors.text))
            .child(self.text)
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
