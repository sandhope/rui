use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_box_style_methods(_input: TokenStream) -> TokenStream {
    let methods = vec![
        (
            "padding_top",
            "pt",
            "Sets the top padding of the element, in pixels.",
        ),
        (
            "padding_right",
            "pr",
            "Sets the right padding of the element, in pixels.",
        ),
        (
            "padding_bottom",
            "pb",
            "Sets the bottom padding of the element, in pixels.",
        ),
        (
            "padding_left",
            "pl",
            "Sets the left padding of the element, in pixels.",
        ),
        (
            "padding_x",
            "px",
            "Sets the horizontal padding of the element, in pixels.",
        ),
        (
            "padding_y",
            "py",
            "Sets the vertical padding of the element, in pixels.",
        ),
        (
            "margin_top",
            "mt",
            "Sets the top margin of the element, in pixels.",
        ),
        (
            "margin_right",
            "mr",
            "Sets the right margin of the element, in pixels.",
        ),
        (
            "margin_bottom",
            "mb",
            "Sets the bottom margin of the element, in pixels.",
        ),
        (
            "margin_left",
            "ml",
            "Sets the left margin of the element, in pixels.",
        ),
        (
            "margin_x",
            "mx",
            "Sets the horizontal margin of the element, in pixels.",
        ),
        (
            "margin_y",
            "my",
            "Sets the vertical margin of the element, in pixels.",
        ),
    ];

    let methods = methods
        .into_iter()
        .map(|(method_name, inner_method, doc_comment)| {
            let method_ident = Ident::new(method_name, proc_macro2::Span::call_site());
            let inner_method_ident = Ident::new(inner_method, proc_macro2::Span::call_site());
            quote! {
                #[doc = #doc_comment]
                fn #method_ident(self, v: f32) -> Self {
                    self.#inner_method_ident(px(v))
                }
            }
        });

    let expanded = quote! {
        #(#methods)*
    };

    TokenStream::from(expanded)
}
