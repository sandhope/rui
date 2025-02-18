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
    { $title:expr; $( $child:expr )* } => {
        {
            let mut section = div().flex().flex_col().p_4().m_4()
                .rounded_md().border_1().border_color(gpui::black())
                .child(div().flex_none().w_full().child($title));
            $(
                section = section.child($child);
            )*
            section
        }
    };
}
