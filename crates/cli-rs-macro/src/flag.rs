use std::fmt;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use thiserror::Error;

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

    quote! {}.into()
}
