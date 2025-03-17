use crate::prelude::*;

#[derive(IntoElement)]
pub struct RootView {
    base: Div,
}

impl RootView {
    pub fn new() -> Self {
        Self { base: div() }
    }
}

impl Styled for RootView {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for RootView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .flex()
            .flex_col()
            .size_full()
            .text_color(cx.theme().colors.text)
            .bg(cx.theme().colors.bg)
    }
}

impl ParentElement for RootView {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}
