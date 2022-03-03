use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, NestedMeta};
use thiserror::Error;

use crate::{attr_meta::extract_meta, doc::extract_doc, kebab_case::upper_camel_to_kebab};

#[derive(Debug, Error)]
enum ArgOptError {
    #[error("#[derive(ArgOpt)] can only be applied to structs with single unnamed field or enums")]
    InvalidStruct,

    #[error("Unexpected type defintion (expected: struct or enum)")]
    UnexpectedTypeDef,
}

fn validate_struct(data_struct: &syn::DataStruct) -> Result<&syn::Field, ArgOptError> {
    let unnamed = match data_struct {
        syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        } => unnamed,

        _ => return Err(ArgOptError::InvalidStruct),
    };

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Ok(field),

        _ => Err(ArgOptError::InvalidStruct),
    }
}

struct ArgOptAttr {
    long: Option<String>,
    short: Option<char>,
}

// HACK: 可読性を上げたい
fn extract_arg_opt_attr<'a>(
    attrs: impl Iterator<Item = &'a Attribute> + 'a,
) -> syn::Result<ArgOptAttr> {
    let mut long: Option<String> = None;
    let mut short: Option<char> = None;

    for nested_meta in extract_meta(attrs, "arg_opt") {
        let meta = match nested_meta {
            NestedMeta::Meta(meta) => meta,
            _ => {
                return Err(syn::Error::new_spanned(
                    nested_meta,
                    "Literals in #[arg_opt(..)] are not allowed",
                ))
            }
        };

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,

            // TODO: default を指定した場合のコード生成を実装する
            syn::Meta::Path(path) if path.is_ident("default") => {
                continue;
            }

            _ => {
                return Err(syn::Error::new_spanned(
                    meta,
                    "Metadata in #[arg_opt(..)] is invalid",
                ))
            }
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => lit,
                _ => {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "#[arg_opt(long = ..)] must be a string literal",
                    ))
                }
            };
            long = Some(lit.value());
        } else if path.is_ident("short") {
            let lit = match lit {
                syn::Lit::Char(lit) => lit,
                _ => {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "#[arg_opt(short = ..)] must be a char literal",
                    ))
                }
            };
            short = Some(lit.value());
        } else {
            return Err(syn::Error::new_spanned(
                path,
                "Unexpected key in #[arg_opt(..)]",
            ));
        }
    }

    Ok(ArgOptAttr { long, short })
}

pub fn derive_arg_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    match &derive_input.data {
        Data::Struct(struct_data) => {
            let field = validate_struct(struct_data)
                .map_err(|err| syn::Error::new_spanned(&derive_input, err.to_string()))?;

            let ArgOptAttr { long, short } = extract_arg_opt_attr(derive_input.attrs.iter())?;
            let short = match short {
                Some(lit) => quote! { Some(cli_rs::ShortFlag::new(#lit)) },
                None => quote! { None },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            let ty = field.ty.clone();
            let struct_name = derive_input.ident;
            let struct_name_kebab_case =
                long.unwrap_or_else(|| upper_camel_to_kebab(&struct_name.to_string()));

            Ok(quote! {
                impl cli_rs::AsArgOpt for #struct_name {
                    fn long() -> cli_rs::LongFlag {
                        cli_rs::LongFlag::new(#struct_name_kebab_case)
                    }

                    fn short() -> Option<cli_rs::ShortFlag> {
                        #short
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        let val = <#ty as std::str::FromStr>::from_str(s).ok()?;
                        Some(#struct_name(val))
                    }
                }
            })
        }

        Data::Enum(_) => {
            let ArgOptAttr { long, short } = extract_arg_opt_attr(derive_input.attrs.iter())?;
            let short = match short {
                Some(lit) => quote! { Some(cli_rs::ShortFlag::new(#lit)) },
                None => quote! { None },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            let enum_name = derive_input.ident;
            let enum_name_kebab_case =
                long.unwrap_or_else(|| upper_camel_to_kebab(&enum_name.to_string()));

            Ok(quote! {
                impl cli_rs::AsArgOpt for #enum_name {
                    fn long() -> cli_rs::LongFlag {
                        cli_rs::LongFlag::new(#enum_name_kebab_case)
                    }

                    fn short() -> Option<cli_rs::ShortFlag> {
                        #short
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        <#enum_name as std::str::FromStr>::from_str(s).ok()
                    }
                }
            })
        }

        _ => Err(syn::Error::new_spanned(
            derive_input,
            format!("{}", ArgOptError::UnexpectedTypeDef),
        )),
    }
}
