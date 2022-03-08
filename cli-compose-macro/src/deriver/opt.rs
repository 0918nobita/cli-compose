mod result;

use convert_case::{Case, Casing};
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;

use crate::doc::extract_doc;

#[derive(FromDeriveInput)]
#[darling(attributes(opt), forward_attrs(doc))]
struct OptInput {
    ident: syn::Ident,

    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    long: Option<String>,

    #[darling(default)]
    short: Option<char>,
}

pub fn derive_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let input = match OptInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => return Ok(err.write_errors()),
    };

    let struct_name = &input.ident;
    let long = input
        .long
        .unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

    let flag = match input.short {
        Some(short) => quote! { cli_compose::Flag::BothLongAndShort(#long.to_owned(), #short) },
        None => quote! { cli_compose::Flag::LongOnly(#long.to_owned()) },
    };

    let doc = extract_doc(&input.attrs);

    Ok(quote! {
        impl cli_compose::AsOpt for #struct_name {
            fn flag() -> cli_compose::Flag {
                #flag
            }

            fn description() -> String {
                #doc.to_owned()
            }
        }
    })
}
