use gpui::{hsla, point, AnyView, AppContext as _, BoxShadow, IntoElement, Render};

use crate::{prelude::*, Text};

pub struct Tooltip {
    title: SharedString,
    meta: Option<SharedString>,
}

impl Tooltip {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            meta: None,
        }
    }

    pub fn simple(title: impl Into<SharedString>, cx: &mut App) -> AnyView {
        cx.new(|_| Self {
            title: title.into(),
            meta: None,
        })
        .into()
    }

    pub fn text(title: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView {
        let title = title.into();
        move |_, cx| {
            cx.new(|_| Self {
                title: title.clone(),
                meta: None,
            })
            .into()
        }
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }
}

impl Render for Tooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let color = cx.theme().colors.text_muted;
        tooltip_container(window, cx, move |el, _, _| {
            el.child(
                h_flex()
                    .gap_4()
                    .child(div().max_w_72().child(self.title.clone())),
            )
            .when_some(self.meta.clone(), |this, meta| {
                this.child(Text::new(meta).text_size(rems_from_px(12.)).color(color))
            })
        })
    }
}

pub fn tooltip_container<V>(
    window: &mut Window,
    cx: &mut Context<V>,
    f: impl FnOnce(Div, &mut Window, &mut Context<V>) -> Div,
) -> impl IntoElement {
    // padding to avoid tooltip appearing right below the mouse cursor
    div().pl_2().pt_2p5().child(
        v_flex()
            .bg(cx.theme().colors.bg_elevated_surface)
            .rounded_lg()
            .border_1()
            .border_color(cx.theme().colors.border_variant)
            .shadow(smallvec::smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., 0.12),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(3.),
                    spread_radius: px(0.),
                },
                BoxShadow {
                    color: hsla(
                        0.,
                        0.,
                        0.,
                        if cx.theme().appearance.is_light() {
                            0.03
                        } else {
                            0.06
                        }
                    ),
                    offset: point(px(1.), px(1.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                }
            ])
            .text_size(rems_from_px(14.))
            .text_color(cx.theme().colors.text)
            .py_1()
            .px_2()
            .map(|el| f(el, window, cx)),
    )
}

pub struct LinkPreview {
    link: SharedString,
}

impl LinkPreview {
    pub fn new(url: &str, cx: &mut App) -> AnyView {
        let mut wrapped_url = String::new();
        for (i, ch) in url.chars().enumerate() {
            if i == 500 {
                wrapped_url.push('…');
                break;
            }
            if i % 100 == 0 && i != 0 {
                wrapped_url.push('\n');
            }
            wrapped_url.push(ch);
        }
        cx.new(|_| LinkPreview {
            link: wrapped_url.into(),
        })
        .into()
    }
}

impl Render for LinkPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text = Text::new(self.link.clone())
            .text_size(rems_from_px(10.))
            .color(cx.theme().colors.text_muted);

        tooltip_container(window, cx, |el, _, _| el.child(text))
    }
}
