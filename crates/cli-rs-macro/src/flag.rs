use std::fmt;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use thiserror::Error;

use crate::{doc::extract_doc, kebab_case::upper_camel_to_kebab};

#[derive(Debug, Error)]
struct InvalidStruct;

impl fmt::Display for InvalidStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#[derive(Flag)] can only be applied to empty structs")
    }
}

fn validate_struct(data: &Data) -> Result<(), InvalidStruct> {
    match data {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => Ok(()),

        _ => Err(InvalidStruct),
    }
}

pub fn derive_flag(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    let doc = extract_doc(derive_input.attrs.into_iter());

    let struct_name = derive_input.ident;
    let struct_name_kebab_case = upper_camel_to_kebab(&struct_name.to_string());

    quote! {
        impl cli_rs::ToArgMeta for #struct_name {
            fn metadata() -> cli_rs::ArgMeta {
                cli_rs::ArgMeta::Flag {
                    long: #struct_name_kebab_case.to_owned(),
                    short: None,
                    description: #doc.to_owned(),
                }
            }
        }
    }
    .into()
}
