use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, Result};

struct MacroInput {
    exprs: Vec<Expr>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut exprs = Vec::new();
        while !input.is_empty() {
            exprs.push(input.parse()?);
        }
        Ok(MacroInput { exprs })
    }
}

fn parse_input(input: ParseStream) -> Result<Vec<Expr>> {
    let mut exprs = Vec::new();
    while !input.is_empty() {
        exprs.push(input.parse()?);
    }
    Ok(exprs)
}

#[proc_macro]
pub fn col(input: TokenStream) -> TokenStream {
    let exprs = parse_macro_input!(input with parse_input);
    // let MacroInput { exprs } = parse_macro_input!(input as MacroInput);

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

#[proc_macro]
pub fn row(input: TokenStream) -> TokenStream {
    // let exprs = parse_macro_input!(input with parse_input);
    let MacroInput { exprs } = parse_macro_input!(input as MacroInput);

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
