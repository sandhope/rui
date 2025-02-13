use crate::prelude::*;

const MASKED: &'static str = "•";

#[derive(IntoElement)]
pub struct Text {
    base: Div,
    text: SharedString,
    marked: bool,
}

impl Text {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            text: text.into(),
            marked: false,
        }
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.marked = masked;
        self
    }
}

impl Styled for Text {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // theme
        // self.base.text_color(cx.theme().foreground)
        let text_display = if self.marked {
            MASKED.repeat(self.text.chars().count())
        } else {
            self.text.to_string()
        };
        self.base.child(text_display)
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
