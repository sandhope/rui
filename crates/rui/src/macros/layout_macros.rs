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
    { $( $label:expr )* } => {
        {
            let mut col = div().flex().flex_col();
            $(
                col = col.child($label);
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
    { $( $label:expr )* } => {
        {
            let mut row = div().flex().flex_row();
            // All elements must be of the same type
            // row = row.children(vec![$($label),*]);
            $(
                row = row.child($label);
            )*
            row
        }
    };
}
