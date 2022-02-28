use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Data, NestedMeta};
use thiserror::Error;

use crate::{attr_meta::extract_meta, doc::extract_doc, kebab_case::upper_camel_to_kebab};

#[derive(Debug, Error)]
enum FlagArgError {
    #[error(
        "#[derive(FlagArg)] can only be applied to structs with single unnamed field or enums"
    )]
    InvalidStruct,

    #[error("Unexpected type defintion (expected: struct or enum)")]
    UnexpectedTypeDef,
}

fn validate_struct(data_struct: &syn::DataStruct) -> Result<&syn::Field, FlagArgError> {
    let unnamed = match data_struct {
        syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        } => unnamed,

        _ => return Err(FlagArgError::InvalidStruct),
    };

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Ok(field),

        _ => Err(FlagArgError::InvalidStruct),
    }
}

struct FlagArgAttr {
    long: Option<String>,
    short: Option<char>,
}

// HACK: 可読性を上げたい
fn extract_flag_arg_attr<'a>(attrs: impl Iterator<Item = &'a Attribute> + 'a) -> FlagArgAttr {
    let mut long: Option<String> = None;
    let mut short: Option<char> = None;

    for nested_meta in extract_meta(attrs, "flag_arg") {
        match nested_meta {
            NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) => {
                    if path.is_ident("long") {
                        let lit = match lit {
                            syn::Lit::Str(lit) => lit,
                            _ => panic!("#[flag_arg(long = ..)] must be a string literal"),
                        };
                        long = Some(lit.value());
                    } else if path.is_ident("short") {
                        let lit = match lit {
                            syn::Lit::Char(lit) => lit,
                            _ => panic!("#[flag_arg(short = ..)] must be a char literal"),
                        };
                        short = Some(lit.value());
                    } else {
                        panic!(
                            "Unexpected key in #[flag_arg(..)]: {}",
                            path.into_token_stream()
                        );
                    }
                }
                _ => panic!("Metadata in #[flag_arg(..)] is invalid"),
            },
            NestedMeta::Lit(_) => {
                panic!("Literals in #[flag_arg(..)] are not allowed")
            }
        }
    }

    FlagArgAttr { long, short }
}

pub fn derive_flag_arg(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    match &derive_input.data {
        Data::Struct(struct_data) => {
            let field = match validate_struct(struct_data) {
                Ok(field) => field,
                Err(err) => return Err(syn::Error::new_spanned(derive_input, format!("{}", err))),
            };

            let FlagArgAttr { long, short } = extract_flag_arg_attr(derive_input.attrs.iter());
            let short = match short {
                Some(lit) => quote! { Some(#lit) },
                None => quote! { None },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            let ty = field.ty.clone();
            let struct_name = derive_input.ident;
            let struct_name_kebab_case =
                long.unwrap_or_else(|| upper_camel_to_kebab(&struct_name.to_string()));

            Ok(quote! {
                impl cli_rs::ToArgMetadatum for #struct_name {
                    fn metadatum() -> cli_rs::ArgMetadatum {
                        cli_rs::ArgMetadatum::FlagArg {
                            long: #struct_name_kebab_case.to_owned(),
                            short: #short,
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
            })
        }
        Data::Enum(_) => Ok(quote! {}),
        _ => Err(syn::Error::new_spanned(
            derive_input,
            format!("{}", FlagArgError::UnexpectedTypeDef),
        )),
    }
}
