use crate::{prelude::*, Button, Headline, HeadlineSize, IconName, ModalView, Text};
use gpui::{
    actions, App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, KeyBinding, Render,
    Window,
};
use smallvec::{smallvec, SmallVec};

actions!(modal, [Escape, Enter]);

const CONTEXT: &str = "Modal";

pub struct Modal {
    focus_handle: FocusHandle,
    title: SharedString,
    primary_button: Button,
    dismiss_button: Button,
    show_close: bool,
    content_builder: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement>>,
}

impl Focusable for Modal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<DismissEvent> for Modal {}

impl ModalView for Modal {}

impl Modal {
    pub fn new(cx: &mut App, title: impl Into<SharedString>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            title: title.into(),
            primary_button: Button::new("Ok").text("Ok"),
            dismiss_button: Button::new("Cancel").text("Cancel").soft(),
            show_close: false,
            content_builder: None,
        }
    }

    pub fn content<F>(mut self, builder: F) -> Self
    where
        F: 'static + Fn(&mut Window, &mut Context<Self>) -> AnyElement,
    {
        self.content_builder = Some(Box::new(builder));
        self
    }

    pub fn bind_keys(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("escape", Escape, Some(CONTEXT)),
            KeyBinding::new("enter", Enter, Some(CONTEXT)),
        ]);
    }

    pub fn show_close(mut self) -> Self {
        self.show_close = true;
        self
    }

    fn cancel(&mut self, _: &Escape, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }
}

impl Render for Modal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .elevation_3(cx)
            .w_96()
            .h_auto()
            .p_4()
            .gap_2()
            // .when(self.show_close, |this| {
            //     this.child(
            //         Button::new("modal-close")
            //             .absolute()
            //             .top_4()
            //             .right_2()
            //             .ghost()
            //             .icon(IconName::Close)
            //             .size(Size::XSmall)
            //             .on_click(cx.listener(move |_, _, window, cx| {
            //                 cx.spawn_in(window, async move |this, cx| {
            //                     this.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
            //                 })
            //                 .detach();
            //             })),
            //     )
            // })
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(Headline::new(self.title.clone()))
                    .when(self.show_close, |this| {
                        this.child(
                            Button::new("modal-close")
                                .ghost()
                                .icon(IconName::Close)
                                .size(Size::XSmall)
                                .on_click(cx.listener(move |_, _, window, cx| {
                                    cx.spawn_in(window, async move |this, cx| {
                                        this.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                                    })
                                    .detach();
                                })),
                        )
                    }),
            )
            .when_some(self.content_builder.as_ref(), |this, builder| {
                this.child(builder(window, cx))
            })
        // .child(
        //     h_flex()
        //         .h(rems(1.75))
        //         .items_center()
        //         .pt_4()
        //         .child(div().flex_1())
        //         .child(
        //             h_flex()
        //                 .items_center()
        //                 .gap_1()
        //                 .child(self.dismiss_button.clone())
        //                 .child(self.primary_button),
        //         )
        //         .into_any_element(),
        // )

        // self.base
        //     .v_flex()
        //     .bg(cx.theme().colors.bg_elevated_surface)
        //     .rounded_lg()
        //     .border_1()
        //     .border_color(cx.theme().colors.border_variant)
        //     .elevation_3(cx)
        //     .w(px(440.))
        //     .p_5()
        //     .child(
        //         v_flex()
        //             .text_size(rems_from_px(14.))
        //             .text_color(cx.theme().colors.text_muted)
        //             .gap_1()
        //             .child(self.header)
        //             .children(self.children),
        //     )
        //     .child(
        //         self.footer.unwrap_or(
        //             h_flex()
        //                 .h(rems(1.75))
        //                 .items_center()
        //                 .pt_4()
        //                 .child(div().flex_1())
        //                 .child(
        //                     h_flex()
        //                         .items_center()
        //                         .gap_1()
        //                         .child(self.dismiss_button)
        //                         .child(self.primary_button),
        //                 )
        //                 .into_any_element(),
        //         ),
        //     )
    }
}
