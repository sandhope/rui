mod layout;
mod style;

use proc_macro::TokenStream;

#[proc_macro]
pub fn row(input: TokenStream) -> TokenStream {
    layout::row(input)
}

#[proc_macro]
pub fn col(input: TokenStream) -> TokenStream {
    layout::col(input)
}

#[proc_macro]
pub fn box_style_methods(input: TokenStream) -> TokenStream {
    style::generate_box_style_methods(input)
}
