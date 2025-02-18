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
            div().flex().flex_col()
            $(
                .child($child)
            )*
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
            // All elements must be of the same type
            // div().flex().flex_row().children(vec![$($child),*])

            // let mut row = div().flex().flex_row();
            // $(
            //     row = row.child($child);
            // )*
            // row

            div().flex().flex_row()
            $(
                .child($child)
            )*
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
    { $title:expr; $( $child:expr )* } => {
        {
            use rui::Card;
            Card::new()
                .child(div().flex_none().w_full().child(Text::new($title)))
            $(
                .child($child)
            )*
        }
    };
}
