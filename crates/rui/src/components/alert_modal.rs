use crate::{prelude::*, Button, Headline, HeadlineSize};
use gpui::{hsla, point, BoxShadow};
use smallvec::{smallvec, SmallVec};

#[derive(IntoElement)]
pub struct AlertModal {
    base: Div,
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    primary_button: Button,
    dismiss_button: Button,
    header: AnyElement,
    footer: Option<AnyElement>,
}

impl AlertModal {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            children: smallvec![],
            primary_button: Button::new("Ok").text("Ok"),
            dismiss_button: Button::new("Cancel").text("Cancel").soft(),
            header: Headline::new(title)
                .size(HeadlineSize::Small)
                .into_any_element(),
            footer: None,
        }
    }

    pub fn primary_button(mut self, primary_button: impl Into<Button>) -> Self {
        self.primary_button = primary_button.into();
        self
    }

    pub fn dismiss_button(mut self, dismiss_button: impl Into<Button>) -> Self {
        self.dismiss_button = dismiss_button.into();
        self
    }

    pub fn header(mut self, header: impl Into<AnyElement>) -> Self {
        self.header = header.into();
        self
    }

    pub fn footer(mut self, footer: impl Into<AnyElement>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}

impl RenderOnce for AlertModal {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_light = cx.theme().appearance.is_light();

        self.base
            .v_flex()
            .id(self.id)
            .bg(cx.theme().colors.bg_elevated_surface)
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().colors.border_variant)
            .shadow(smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., if is_light { 0.06 } else { 0.12 }),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(3.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., if is_light { 0.06 } else { 0.08 }),
                    offset: point(px(0.), px(3.)),
                    blur_radius: px(6.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.04),
                    offset: point(px(0.), px(6.)),
                    blur_radius: px(12.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., if is_light { 0.04 } else { 0.12 }),
                    offset: point(px(1.), px(1.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                },
            ])
            .w(px(440.))
            .p_5()
            .child(
                v_flex()
                    .text_size(rems_from_px(14.))
                    .text_color(cx.theme().colors.text_muted)
                    .gap_1()
                    .child(self.header)
                    .children(self.children),
            )
            .child(
                self.footer.unwrap_or(
                    h_flex()
                        .h(rems(1.75))
                        .items_center()
                        .pt_4()
                        .child(div().flex_1())
                        .child(
                            h_flex()
                                .items_center()
                                .gap_1()
                                .child(self.dismiss_button)
                                .child(self.primary_button),
                        )
                        .into_any_element(),
                ),
            )
    }
}

impl ParentElement for AlertModal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for AlertModal {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}
