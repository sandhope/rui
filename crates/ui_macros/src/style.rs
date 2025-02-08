extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_box_style_methods(_input: TokenStream) -> TokenStream {
    let methods = vec![
        ("padding_top", "pt"),
        ("padding_right", "pr"),
        ("padding_bottom", "pb"),
        ("padding_left", "pl"),
        ("padding_x", "px"),
        ("padding_y", "py"),
        ("margin_top", "mt"),
        ("margin_right", "mr"),
        ("margin_bottom", "mb"),
        ("margin_left", "ml"),
        ("margin_x", "mx"),
        ("margin_y", "my"),
    ];

    let methods = methods.into_iter().map(|(method_name, inner_method)| {
        let method_ident = Ident::new(method_name, proc_macro2::Span::call_site());
        let inner_method_ident = Ident::new(inner_method, proc_macro2::Span::call_site());
        quote! {
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
