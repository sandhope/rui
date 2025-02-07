use gpui::{div, App, Div, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window};

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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // theme
        // self.base.text_color(cx.theme().foreground)
        self.base.child(self.text)
    }
}
