use crate::prelude::*;
use gpui::UnderlineStyle;

#[derive(IntoElement)]
pub struct Text {
    base: Div,
    text: SharedString,
    color: Option<Hsla>,
    strikethrough: bool,
    underline: bool,
    single_line: bool,
    truncate: bool,
    masked: bool,
}

impl Text {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            text: text.into(),
            color: None,
            strikethrough: false,
            underline: false,
            single_line: false,
            truncate: false,
            masked: false,
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Truncates overflowing text with an ellipsis (`…`) if needed.
    pub fn truncate(mut self) -> Self {
        self.truncate = true;
        self
    }

    pub fn single_line(mut self) -> Self {
        self.text = SharedString::from(self.text.replace('\n', "⏎"));
        self.single_line = true;
        self
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }
}

impl Styled for Text {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

const MASKED: &'static str = "•";

impl RenderOnce for Text {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base
            .when(self.underline, |mut this| {
                this.text_style()
                    .get_or_insert_with(Default::default)
                    .underline = Some(UnderlineStyle {
                    thickness: px(1.),
                    color: None,
                    wavy: false,
                });
                this
            })
            .when(self.strikethrough, |this| this.line_through())
            .when(self.single_line, |this| this.whitespace_nowrap())
            .when(self.truncate, |this| {
                this.overflow_x_hidden().text_ellipsis()
            })
            .text_color(self.color.unwrap_or(window.text_style().color))
            .child(if self.masked {
                SharedString::from(MASKED.repeat(self.text.chars().count()))
            } else {
                self.text
            })
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
