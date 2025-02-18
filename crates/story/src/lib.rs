use gpui::{div, App, Div, IntoElement, ParentElement, Styled as _};

use rui::h_flex;

pub fn section(title: impl IntoElement, cx: &mut App) -> Div {
    use rui::ActiveTheme;

    h_flex()
        .items_center()
        .gap_4()
        .p_4()
        .w_full()
        .rounded_lg()
        //.rounded(cx.theme().radius)
        .border_1()
        .border_color(cx.theme().colors.border)
        .border_color(gpui::black())
        .flex_wrap()
        .justify_around()
        .child(div().flex_none().w_full().child(title))
}
