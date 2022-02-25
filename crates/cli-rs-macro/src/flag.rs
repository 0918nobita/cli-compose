use std::fmt;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Data};
use thiserror::Error;

use crate::{attr_meta::extract_meta, doc::extract_doc, kebab_case::upper_camel_to_kebab};

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

struct FlagAttr {
    long: Option<String>,
    short: Option<char>,
}

// HACK: 可読性を上げたい
fn extract_flag_attr<'a>(attrs: impl Iterator<Item = &'a Attribute> + 'a) -> FlagAttr {
    let mut long: Option<String> = None;
    let mut short: Option<char> = None;

    for metadata in extract_meta(attrs, "flag") {
        match metadata {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) => {
                    if path.is_ident("long") {
                        let lit = match lit {
                            syn::Lit::Str(lit) => lit,
                            _ => panic!("#[flag(long = ..)] must be a string literal"),
                        };
                        long = Some(lit.value());
                    } else if path.is_ident("short") {
                        let lit = match lit {
                            syn::Lit::Char(lit) => lit,
                            _ => panic!("#[flag(short = ..)] must be a char literal"),
                        };
                        short = Some(lit.value());
                    } else {
                        panic!(
                            "Unexpected key in #[flag(..)]: {}",
                            path.into_token_stream()
                        );
                    }
                }
                _ => panic!("Metadata in flag attribute is invalid"),
            },
            syn::NestedMeta::Lit(_) => {
                panic!("Literals in flag attribute are not supported")
            }
        }
    }

    FlagAttr { long, short }
}

pub fn derive_flag(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    let FlagAttr { long, short } = extract_flag_attr(derive_input.attrs.iter());
    let short = match short {
        Some(lit) => quote! { Some(#lit) },
        None => quote! { None },
    };

    let doc = extract_doc(derive_input.attrs.into_iter());

    let struct_name = derive_input.ident;
    let struct_name_kebab_case =
        long.unwrap_or_else(|| upper_camel_to_kebab(&struct_name.to_string()));

    quote! {
        impl cli_rs::ToArgMeta for #struct_name {
            fn metadata() -> cli_rs::ArgMeta {
                cli_rs::ArgMeta::Flag {
                    long: #struct_name_kebab_case.to_owned(),
                    short: #short,
                    description: #doc.to_owned(),
                }
            }
        }
    }
    .into()
}
