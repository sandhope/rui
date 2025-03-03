use crate::{prelude::*, Icon};

#[derive(Default)]
enum IndicatorKind {
    #[default]
    Dot,
    Bar,
    Icon(Icon),
}

#[derive(IntoElement)]
pub struct Indicator {
    kind: IndicatorKind,
    border_color: Option<Hsla>,
    pub color: Option<Hsla>,
}

impl Indicator {
    pub fn dot() -> Self {
        Self {
            kind: IndicatorKind::Dot,
            border_color: None,
            color: None,
        }
    }

    pub fn bar() -> Self {
        Self {
            kind: IndicatorKind::Bar,
            border_color: None,
            color: None,
        }
    }

    pub fn icon(icon: impl Into<Icon>) -> Self {
        Self {
            kind: IndicatorKind::Icon(icon.into()),
            border_color: None,
            color: None,
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn border_color(mut self, color: impl Into<Hsla>) -> Self {
        self.border_color = Some(color.into());
        self
    }
}

impl RenderOnce for Indicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let container = div()
            .flex_none()
            .when_some(self.border_color, |this, color| {
                this.when(
                    matches!(self.kind, IndicatorKind::Dot | IndicatorKind::Bar),
                    |this| this.border_1().border_color(color),
                )
            });

        let color = self.color.unwrap_or(cx.theme().colors.primary);

        match self.kind {
            IndicatorKind::Icon(icon) => {
                container.child(icon.map(|icon| icon.custom_size(rems_from_px(8.)).color(color)))
            }
            IndicatorKind::Dot => container.w_1p5().h_1p5().rounded_full().bg(color),
            IndicatorKind::Bar => container.w_full().h_1p5().rounded_t_md().bg(color),
        }
    }
}
