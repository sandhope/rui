use gpui::{div, Div, IntoElement, ParentElement, Styled as _, Window};

use rui::h_flex;

pub fn section(title: impl IntoElement, _cx: &Window) -> Div {
    //use ui::ActiveTheme;
    //let theme = cx.theme();

    h_flex()
        .items_center()
        .gap_4()
        .p_4()
        .w_full()
        .rounded_lg()
        //.rounded(cx.theme().radius)
        .border_1()
        //.border_color(theme.border)
        .flex_wrap()
        .justify_around()
        .child(div().flex_none().w_full().child(title))
}
