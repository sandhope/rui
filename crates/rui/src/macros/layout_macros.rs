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

            div().flex().flex_row().items_center()
            $(
                .child($child)
            )*
        }
    };
}

/// A macro to create a root layout with multiple children.
///
/// This macro takes a list of expressions and adds them as children to a root layout.
///
/// # Example
///
/// ```rust
/// Root! {
///     child1
///     child2
///     child3
/// }
/// ```
#[macro_export]
macro_rules! Root {
    { $( $child:expr )* } => {
        {
            RootView::new()
            $(
                .child($child)
            )*
        }
    };
}

/// A macro to create a section layout with a title and multiple children.
///
/// This macro allows you to create a layout section that includes a title and a list of child components.
/// The title can be given as a string or omitted, in which case the section will appear without a title.
///
/// The layout applies the following default styles to the section:
/// - Padding: `px(16.0)`
/// - Margin: `(16.0, 0.0)`
/// - Border Radius: `px(4.0)`
/// - Border Width: `px(1.0)`

/// # Example
/// Using the macro with a title:
/// ```rust
/// Section! {
///     "My Section";
///     child1
///     child2
///     child3
/// }
/// ```
///
/// Using the macro without a title:
/// ```rust
/// Section! {
///     child1
///     child2
///     child3
/// }
/// ```
#[macro_export]
macro_rules! Section {
    { $( $child:expr )* } => {
        {
            Card::new()
            $(
                .child($child)
            )*
        }
    };
    { $title:expr; $( $child:expr )* } => {
        {
            Card::new()
                .title($title)
            $(
                .child($child)
            )*
        }
    };
}
