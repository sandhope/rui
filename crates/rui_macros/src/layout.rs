use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, LitStr, Result, Token};

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

    let child = exprs.iter();

    let expanded = quote! {
        {
            div().flex().flex_col()
            #(
                .child(#child)
            )*
        }
    };

    TokenStream::from(expanded)
}

pub fn row(input: TokenStream) -> TokenStream {
    // let exprs = parse_macro_input!(input with parse_input);
    let LayoutInput { exprs } = parse_macro_input!(input as LayoutInput);

    let child = exprs.iter();

    let expanded = quote! {
        {
            div().flex().flex_row()
            #(
                .child(#child)
            )*
        }
    };

    TokenStream::from(expanded)
}

struct SectionInput {
    title: Option<LitStr>,
    children: Vec<Expr>,
}

impl Parse for SectionInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let title = if input.peek(LitStr) {
            let t = input.parse::<LitStr>()?;
            input.parse::<Token![;]>()?;
            Some(t)
        } else {
            None
        };

        let mut children = Vec::new();
        while !input.is_empty() {
            children.push(input.parse()?);
        }
        Ok(SectionInput { title, children })
    }
}

pub fn section(input: TokenStream) -> TokenStream {
    let SectionInput { title, children } = parse_macro_input!(input as SectionInput);

    // let children = children.iter().map(|child| {
    //     quote! {
    //         .child(#child)
    //     }
    // });
    // let title = title.iter().map(|t| quote! { .title(#t) });
    // let expanded = quote! {
    //     {
    //         Card::new()
    //         #(#title)*
    //         #(#children)*
    //     }
    // };

    let child = children.iter();
    let title = title.iter();

    let expanded = quote! {
        {
            Card::new()
            #(
                .title(#title)
            )*
            #(
                .child(#child)
            )*
        }
    };

    TokenStream::from(expanded)
}
