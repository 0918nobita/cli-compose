use std::fmt;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct InvalidStruct;

impl fmt::Display for InvalidStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#[derive(Flag)] can only be applied to structs with single unnamed field whose type is `bool`",
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
        [field @ syn::Field {
            ty: syn::Type::Path(syn::TypePath { path, .. }),
            ..
        }] => {
            let segments = path.segments.iter().collect::<Vec<_>>();
            match segments[..] {
                [syn::PathSegment { ident, .. }] if ident == "bool" => Ok(field),
                _ => Err(InvalidStruct),
            }
        }

        _ => Err(InvalidStruct),
    }
}

pub fn derive_flag(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let _field = validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    quote! {}.into()
}
