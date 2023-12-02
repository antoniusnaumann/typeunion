use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Attribute, Ident, Token, Visibility};

struct TypeItem {
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    cases: Punctuated<Ident, Token![+]>,
}

impl Parse for TypeItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let _ = input.parse::<Token![type]>()?;
        let name = input.parse()?;
        let _ = input.parse::<Token![=]>()?;
        let mut cases = Punctuated::new();

        loop {
            cases.push_value(input.parse()?);
            if input.peek(Token![;]) {
                let _ = input.parse::<Token![;]>()?;
                break;
            }
            cases.push_punct(input.parse()?);
        }

        Ok(Self {
            attrs,
            vis,
            name,
            cases,
        })
    }
}

struct Args {
    superset: Option<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(if let Ok(_) = input.parse::<Token![super]>() {
            let _ = input.parse::<Token![=]>()?;
            Self {
                superset: input.parse()?,
            }
        } else {
            Self { superset: None }
        })
    }
}

#[proc_macro_attribute]
pub fn type_union(attr: TokenStream, item: TokenStream) -> TokenStream {
    let Args { superset } = parse_macro_input!(attr as Args);
    let TypeItem {
        attrs,
        vis,
        name,
        cases,
    } = parse_macro_input!(item as TypeItem);
    let cases = cases.into_iter().map(|ident| ident).collect::<Vec<_>>();

    let impls = if let Some(superset) = superset {
        quote! {
            impl From<#name> for #superset {
                fn from(value: #name) -> Self {
                    match value {
                        #(#name::#cases(case) => #superset::#cases(case)),*
                    }
                }
            }
        }
    } else {
        quote!()
    };

    quote! {
        #(#attrs)*
        #vis enum #name {
            #(#cases(#cases)),*
        }

        #impls

        #(
            impl From<#cases> for #name {
                fn from(value: #cases) -> Self {
                    #name::#cases(value)
                }
            }
        )*
    }
    .into()
}
