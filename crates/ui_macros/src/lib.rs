mod layout;
mod style;

use proc_macro::TokenStream;

/// A macro to create a row layout with multiple children.
/// 
/// This macro takes a list of expressions and adds them as children to a row layout.
/// 
/// # Example
/// 
/// ```rust
/// row! {
///     child1
///     child2
///     child3
/// }
/// ```
#[proc_macro]
pub fn row(input: TokenStream) -> TokenStream {
    layout::row(input)
}

/// A macro to create a column layout with multiple children.
/// 
/// This macro takes a list of expressions and adds them as children to a column layout.
/// 
/// # Example
/// 
/// ```rust
/// rol! {
///     child1
///     child2
///     child3
/// }
/// ```
#[proc_macro]
pub fn col(input: TokenStream) -> TokenStream {
    layout::col(input)
}

/// Generates methods for box styles.
#[proc_macro]
pub fn box_style_methods(input: TokenStream) -> TokenStream {
    style::generate_box_style_methods(input)
}
