use std::fmt;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct InvalidStruct;

impl fmt::Display for InvalidStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#[derive(Arg)] can only be applied to structs with single unnamed field",
        )
    }
}

pub fn validate_struct(data: &Data) -> Result<&syn::Field, InvalidStruct> {
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

fn try_get_single_line_doc(attr: Attribute) -> Option<String> {
    let tokens = match attr {
        Attribute {
            path,
            style: syn::AttrStyle::Outer,
            tokens,
            ..
        } if path.is_ident("doc") => tokens,
        _ => return None,
    };
    let doc = tokens
        .into_iter()
        .skip(1)
        .collect::<proc_macro2::TokenStream>();
    Some(format!("{}", doc).trim_matches('"').trim_start().to_owned())
}

fn extract_doc(attrs: impl Iterator<Item = Attribute>) -> String {
    attrs
        .filter_map(try_get_single_line_doc)
        .collect::<Vec<_>>()
        .join("\n")
}

fn upper_camel_to_kebab(str: &str) -> String {
    fn folder((first_skipped, mut cs): (bool, Vec<char>), c: char) -> (bool, Vec<char>) {
        if ('A'..='Z').contains(&c) {
            if !first_skipped {
                cs.push(c.to_ascii_lowercase());
                return (true, cs);
            }
            cs.push('-');
            cs.push(c.to_ascii_lowercase());
            return (true, cs);
        }

        cs.push(c);
        (first_skipped, cs)
    }

    str.chars()
        .fold((false, vec![]), folder)
        .1
        .into_iter()
        .collect()
}

pub fn derive_arg(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let doc = extract_doc(derive_input.attrs.into_iter());

    let field = validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    let ty = field.ty.clone();
    let struct_name = derive_input.ident;
    let struct_name_lowercase = upper_camel_to_kebab(&struct_name.to_string());

    quote! {
        impl cli_rs::ToArg for #struct_name {
            fn name() -> String {
                #struct_name_lowercase.to_owned()
            }

            fn description() -> String {
                #doc.to_owned()
            }

            fn parse(str: &str) -> Option<Self> {
                use std::str::FromStr;
                #ty::from_str(str).ok().map(|v| #struct_name(v))
            }
        }
    }
    .into()
}
