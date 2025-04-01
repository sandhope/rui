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
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
    primary_button: Button,
    dismiss_button: Button,
    header: AnyElement,
    footer: Option<AnyElement>,
    show_close: bool,
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
            base: div(),
            children: smallvec![],
            primary_button: Button::new("Ok").text("Ok"),
            dismiss_button: Button::new("Cancel").text("Cancel").soft(),
            header: Headline::new(title)
                .size(HeadlineSize::Small)
                .into_any_element(),
            footer: None,
            show_close: false,
        }
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
        // let is_light = cx.theme().appearance.is_light();

        v_flex()
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::cancel))
            .on_action(move |_: &Escape, _window, _cx| println!("xxx"))
            .elevation_3(cx)
            .w_96()
            .h_auto()
            .p_4()
            .gap_2()
            .when(self.show_close, |this| {
                this.child(
                    Button::new(SharedString::from("modal-close"))
                        .absolute()
                        .top_2()
                        .right_2()
                        .ghost()
                        .icon(IconName::Close)
                        .ghost()
                        .size(Size::XSmall)
                        .on_click(move |_, _window, _cx| {
                            // window.close_modal(cx);
                            //
                        }),
                )
            })
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(Headline::new("Give Feedback")),
            )

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

impl ParentElement for Modal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for Modal {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}
