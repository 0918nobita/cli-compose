use std::fmt;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Data;
use thiserror::Error;

use crate::{attr_meta::extract_meta, doc::extract_doc, kebab_case::upper_camel_to_kebab};

#[derive(Debug, Error)]
struct InvalidStruct;

impl fmt::Display for InvalidStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#[derive(Arg)] can only be applied to structs with single unnamed field",
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

struct ArgAttr {
    name: Option<String>,
}

// HACK: 可読性を上げたい
fn extract_arg_attr<'a>(attrs: impl Iterator<Item = &'a syn::Attribute> + 'a) -> ArgAttr {
    let mut name: Option<String> = None;

    for metadata in extract_meta(attrs, "arg") {
        match metadata {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) => {
                    if path.is_ident("name") {
                        let lit = match lit {
                            syn::Lit::Str(lit) => lit,
                            _ => panic!("#[arg(name = ..)] must be a string literal"),
                        };
                        name = Some(lit.value());
                    } else {
                        panic!("Unexpected key in #[arg(..)]: {}", path.into_token_stream());
                    }
                }
                _ => panic!("Metadata in #[arg(..)] is invalid"),
            },
            syn::NestedMeta::Lit(_) => {
                panic!("Literals in #[arg(..)] are not supported")
            }
        }
    }

    ArgAttr { name }
}

pub fn derive_arg(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let field = validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    let ArgAttr { name } = extract_arg_attr(derive_input.attrs.iter());

    let doc = extract_doc(derive_input.attrs.iter());

    let _ty = field.ty.clone();
    let struct_name = derive_input.ident;
    let struct_name_kebab_case =
        name.unwrap_or_else(|| upper_camel_to_kebab(&struct_name.to_string()));

    quote! {
        impl cli_rs::ToArgMeta for #struct_name {
            fn metadata() -> cli_rs::ArgMeta {
                cli_rs::ArgMeta::Arg {
                    name: #struct_name_kebab_case.to_owned(),
                    description: #doc.to_owned(),
                }
            }
        }
    }
    .into()
}
