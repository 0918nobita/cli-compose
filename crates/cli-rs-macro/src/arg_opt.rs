mod result;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Data, NestedMeta};

use self::result::{ArgOptError, ArgOptResult};
use crate::{attr_meta::extract_meta, doc::extract_doc, kebab_case::upper_camel_to_kebab};

fn validate_struct(data_struct: &syn::DataStruct) -> Option<&syn::Field> {
    let unnamed = match data_struct {
        syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        } => unnamed,

        _ => return None,
    };

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Some(field),

        _ => None,
    }
}

struct ArgOptAttr {
    long: Option<String>,
    short: Option<char>,
}

// HACK: 可読性を上げたい
fn extract_arg_opt_attr<'a, A>(attrs: A) -> ArgOptResult<ArgOptAttr>
where
    A: Iterator<Item = &'a Attribute> + 'a,
{
    let mut long: Option<String> = None;
    let mut short: Option<char> = None;

    for nested_meta in extract_meta(attrs, "arg_opt") {
        let meta = match nested_meta {
            NestedMeta::Meta(meta) => meta,
            _ => return Err(ArgOptError::UnexpectedLit(nested_meta.to_token_stream())),
        };

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,

            // TODO: default を指定した場合のコード生成を実装する
            syn::Meta::Path(path) if path.is_ident("default") => {
                continue;
            }

            _ => return Err(ArgOptError::InvalidMeta(meta.to_token_stream())),
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => lit,
                _ => return Err(ArgOptError::InvalidLongValue(lit.to_token_stream())),
            };
            long = Some(lit.value());
        } else if path.is_ident("short") {
            let lit = match lit {
                syn::Lit::Char(lit) => lit,
                _ => return Err(ArgOptError::InvalidShortValue(lit.to_token_stream())),
            };
            short = Some(lit.value());
        } else {
            return Err(ArgOptError::UnexpectedKey(path.to_token_stream()));
        }
    }

    Ok(ArgOptAttr { long, short })
}

fn codegen(derive_input: &syn::DeriveInput) -> ArgOptResult<TokenStream> {
    match &derive_input.data {
        Data::Struct(struct_data) => {
            let field = validate_struct(struct_data)
                .ok_or_else(|| ArgOptError::InvalidTypeDef(derive_input.to_token_stream()))?;

            let ArgOptAttr { long, short } = extract_arg_opt_attr(derive_input.attrs.iter())?;
            let short = match short {
                Some(lit) => quote! { Some(cli_rs::ShortFlag::new(#lit)) },
                None => quote! { None },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            let ty = field.ty.clone();
            let struct_name = &derive_input.ident;
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

            let enum_name = &derive_input.ident;
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

        _ => Err(ArgOptError::InvalidTypeDef(derive_input.to_token_stream())),
    }
}

pub fn derive_arg_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;
    codegen(&derive_input).map_err(|err| err.into())
}
