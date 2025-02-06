use gpui::{div, Div, IntoElement, ParentElement, RenderOnce, SharedString, Styled, WindowContext};

// use crate::{h_flex, ActiveTheme};

#[derive(IntoElement)]
pub struct Text {
    base: Div,
    text: SharedString,
}

impl Text {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            text: text.into(),
        }
    }
}

impl Styled for Text {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Text {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.base
            // .text_color(cx.theme().foreground)
            .child(self.text)
    }
}
