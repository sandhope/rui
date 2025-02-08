use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, Result};

struct LayoutInput {
    exprs: Vec<Expr>,
}

impl Parse for LayoutInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut exprs = Vec::new();
        while !input.is_empty() {
            exprs.push(input.parse()?);
        }
        Ok(LayoutInput { exprs })
    }
}

fn parse_input(input: ParseStream) -> Result<Vec<Expr>> {
    let mut exprs = Vec::new();
    while !input.is_empty() {
        exprs.push(input.parse()?);
    }
    Ok(exprs)
}

pub fn col(input: TokenStream) -> TokenStream {
    let exprs = parse_macro_input!(input with parse_input);
    // let LayoutInput { exprs } = parse_macro_input!(input as LayoutInput);

    let output = exprs.iter().map(|expr| {
        quote! {
            col = col.child(#expr);
        }
    });

    let expanded = quote! {
        {
            let mut col = div().flex().flex_col();
            #(#output)*
            col
        }
    };

    TokenStream::from(expanded)
}

pub fn row(input: TokenStream) -> TokenStream {
    // let exprs = parse_macro_input!(input with parse_input);
    let LayoutInput { exprs } = parse_macro_input!(input as LayoutInput);

    let output = exprs.iter().map(|expr| {
        quote! {
            col = col.child(#expr);
        }
    });

    let expanded = quote! {
        {
            let mut col = div().flex().flex_row();
            #(#output)*
            col
        }
    };

    TokenStream::from(expanded)
}
