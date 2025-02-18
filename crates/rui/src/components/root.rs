use crate::prelude::*;
use gpui::AnyElement;

#[derive(IntoElement)]
pub struct Root {
    base: Div,
}

impl Root {
    pub fn new() -> Self {
        Self { base: div() }
    }
}

impl Styled for Root {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Root {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors.bg)
    }
}

impl ParentElement for Root {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}
