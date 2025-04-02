mod derive_path_str;
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

/// A macro to create a section layout with a title and multiple children.
///
/// This macro allows you to create a layout section that includes a title and a list of child components.
/// The title can be given as a string or omitted, in which case the section will appear without a title.
///
/// The layout applies the following default styles to the section:
/// - Padding: `16.0 px`
/// - Margin: `(16.0, 0.0, 16.0, 0.0) px`
/// - Border Radius: `4.0 px`
/// - Border Width: `1.0 px`

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
#[proc_macro]
pub fn section(input: TokenStream) -> TokenStream {
    layout::section(input)
}

/// Generates methods for box styles.
#[proc_macro]
pub fn box_style_methods(input: TokenStream) -> TokenStream {
    style::generate_box_style_methods(input)
}

/// Derives the `path` method for an enum.
///
/// This macro generates a `path` method for each variant of the enum, which returns a string
/// representation of the enum variant's path. The path is constructed using a prefix and
/// optionally a suffix, which are specified using attributes.
///
/// # Attributes
///
/// - `#[path_str(prefix = "...")]`: Required. Specifies the prefix for all paths.
/// - `#[path_str(suffix = "...")]`: Optional. Specifies a suffix for all paths.
/// - `#[strum(serialize_all = "...")]`: Optional. Specifies the case conversion for variant names.
///
/// # Example
///
/// ```
/// use strum::EnumString;
/// use ui_macros::{path_str, DerivePathStr};
///
/// #[derive(EnumString, DerivePathStr)]
/// #[path_str(prefix = "my_prefix", suffix = ".txt")]
/// #[strum(serialize_all = "snake_case")]
/// enum MyEnum {
///     VariantOne,
///     VariantTwo,
/// }
///
/// // These assertions would work if we could instantiate the enum
/// // assert_eq!(MyEnum::VariantOne.path(), "my_prefix/variant_one.txt");
/// // assert_eq!(MyEnum::VariantTwo.path(), "my_prefix/variant_two.txt");
/// ```
///
/// # Panics
///
/// This macro will panic if used on anything other than an enum.
#[proc_macro_derive(DerivePathStr, attributes(path_str))]
pub fn derive_path_str(input: TokenStream) -> TokenStream {
    derive_path_str::derive_path_str(input)
}

/// A marker attribute for use with `DerivePathStr`.
///
/// This attribute is used to specify the prefix and suffix for the `path` method
/// generated by `DerivePathStr`. It doesn't modify the input and is only used as a
/// marker for the derive macro.
#[proc_macro_attribute]
pub fn path_str(_args: TokenStream, input: TokenStream) -> TokenStream {
    // This attribute doesn't modify the input, it's just a marker
    input
}
