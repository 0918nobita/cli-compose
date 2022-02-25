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
        write!(
            f,
            "#[derive(FlagArg)] can only be applied to structs with single unnamed field"
        )
    }
}

fn validate_struct(data: &Data) -> Result<&syn::Field, InvalidStruct> {
    let unnamed = match data {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        }) => unnamed,

        _ => return Err(InvalidStruct),
    };

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Ok(field),

        _ => Err(InvalidStruct),
    }
}

pub fn derive_flag_arg(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let field = validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    let doc = extract_doc(derive_input.attrs.iter());

    let ty = field.ty.clone();
    let struct_name = derive_input.ident;
    let struct_name_kebab_case = upper_camel_to_kebab(&struct_name.to_string());

    quote! {
        impl cli_rs::ToArgMeta for #struct_name {
            fn metadata() -> cli_rs::ArgMeta {
                cli_rs::ArgMeta::FlagArg {
                    long: #struct_name_kebab_case.to_owned(),
                    short: None,
                    description: #doc.to_owned(),
                }
            }
        }

        impl cli_rs::AsFlagArg for #struct_name {
            fn parse(s: &str) -> Option<Self> {
                let val = <#ty as std::str::FromStr>::from_str(s).ok()?;
                Some(#struct_name(val))
            }
        }
    }
    .into()
}
