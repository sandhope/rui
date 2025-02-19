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

    let output = exprs.iter().map(|expr| {
        quote! {
            .child(#expr)
        }
    });

    let expanded = quote! {
        {
            div().flex().flex_col()
            #(#output)*
        }
    };

    TokenStream::from(expanded)
}

pub fn row(input: TokenStream) -> TokenStream {
    // let exprs = parse_macro_input!(input with parse_input);
    let LayoutInput { exprs } = parse_macro_input!(input as LayoutInput);

    let output = exprs.iter().map(|expr| {
        quote! {
            .child(#expr)
        }
    });

    let expanded = quote! {
        {
            div().flex().flex_row()
            #(#output)*
        }
    };

    TokenStream::from(expanded)
}

pub fn root(input: TokenStream) -> TokenStream {
    // let exprs = parse_macro_input!(input with parse_input);
    let LayoutInput { exprs } = parse_macro_input!(input as LayoutInput);

    let output = exprs.iter().map(|expr| {
        quote! {
            .child(#expr)
        }
    });

    let expanded = quote! {
        {
            RootView::new()
            #(#output)*
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

    let output = children.iter().map(|child| {
        quote! {
            .child(#child)
        }
    });

    let title_iter = title.iter();

    let expanded = quote! {
        {
            Card::new()
            #(
                .title(#title_iter)
            )*
            #(#output)*
        }
    };

    TokenStream::from(expanded)
}
