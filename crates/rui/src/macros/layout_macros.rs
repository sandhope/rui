/// A macro to create a column layout with multiple children.
///
/// This macro takes a list of expressions and adds them as children to a column layout.
///
/// # Example
///
/// ```rust
/// Col! {
///     child1
///     child2
///     child3
/// }
/// ```
#[macro_export]
macro_rules! Col {
    { $( $child:expr )* } => {
        {
            let mut col = div().flex().flex_col();
            $(
                col = col.child($child);
            )*
            col
        }
    };
}

/// A macro to create a row layout with multiple children.
///
/// This macro takes a list of expressions and adds them as children to a row layout.
///
/// # Example
///
/// ```rust
/// Row! {
///     child1
///     child2
///     child3
/// }
/// ```
#[macro_export]
macro_rules! Row {
    { $( $child:expr )* } => {
        {
            let mut row = div().flex().flex_row();
            // All elements must be of the same type
            // row = row.children(vec![$($child),*]);
            $(
                row = row.child($child);
            )*
            row
        }
    };
}

/// A macro to create a section layout with a title and multiple children.
///
/// This macro takes a title and a list of expressions and adds them as children to a section layout.
///
/// # Example
///
/// ```rust
/// Section! {
///     "title";
///     child1
///     child2
///     child3
/// }
/// ```
#[macro_export]
macro_rules! Section {
    { $title:expr; $( $label:expr )* } => {
        {
            //use ui::ActiveTheme;
            //let theme = cx.theme();
            let mut section = div().h_flex()
                .items_center()
                .gap_4()
                .p_4()
                .w_full()
                .rounded_lg()
                //.rounded(cx.theme().radius)
                .border_1()
                //.border_color(theme.border)
                .border_color(gpui::black())
                .flex_wrap()
                .justify_around()
                .child(div().flex_none().w_full().child($title));
            $(
                section = section.child($label);
            )*
            section
        }
    };
}
