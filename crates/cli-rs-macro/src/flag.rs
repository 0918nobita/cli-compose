use std::fmt;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, NestedMeta};
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
fn extract_flag_attr<'a>(attrs: impl Iterator<Item = &'a Attribute> + 'a) -> syn::Result<FlagAttr> {
    let mut long: Option<String> = None;
    let mut short: Option<char> = None;

    for nested_meta in extract_meta(attrs, "flag") {
        let meta = match nested_meta {
            NestedMeta::Meta(meta) => meta,
            _ => {
                return Err(syn::Error::new_spanned(
                    nested_meta,
                    "Literals in #[flag(..)] are not allowed",
                ))
            }
        };

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,
            _ => {
                return Err(syn::Error::new_spanned(
                    meta,
                    "Metadata in #[flag(..)] is invalid",
                ))
            }
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => lit,
                _ => {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "#[flag(long = ..)] must be a string literal",
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
                        "#[flag(short = ..)] must be a char literal",
                    ))
                }
            };
            short = Some(lit.value());
        } else {
            return Err(syn::Error::new_spanned(
                path,
                "Unexpected key in #[flag(..)]",
            ));
        }
    }

    Ok(FlagAttr { long, short })
}

pub fn derive_flag(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    let FlagAttr { long, short } = extract_flag_attr(derive_input.attrs.iter())?;
    let short = match short {
        Some(lit) => quote! { Some(cli_rs::ShortFlag::new(#lit)) },
        None => quote! { None },
    };

    let doc = extract_doc(derive_input.attrs.iter());

    let struct_name = derive_input.ident;
    let struct_name_kebab_case =
        long.unwrap_or_else(|| upper_camel_to_kebab(&struct_name.to_string()));

    Ok(quote! {
        impl cli_rs::AsFlag for #struct_name {
            fn long() -> cli_rs::LongFlag {
                cli_rs::LongFlag::new(#struct_name_kebab_case)
            }

            fn short() -> Option<cli_rs::ShortFlag> {
                #short
            }

            fn description() -> String {
                #doc.to_owned()
            }
        }
    })
}

#[derive(Debug, Error)]
pub enum FlagError {
    #[error("Duplicate short flag: `{0:?} -> {1:?}` and `{0:?} -> {2:?}`")]
    DuplicateShortFlag(char, String, String),
}

/// [`FlagNormalizer`] を使ってのみ生成可能な、正規化済みのフラグ
///
/// 短縮フラグが定義されている場合はそれも含む
#[derive(Hash)]
pub struct NormalizedFlag {
    short: Option<char>,
    long: String,
}

impl NormalizedFlag {
    #[allow(dead_code)]
    pub const fn short(&self) -> &Option<char> {
        &self.short
    }

    #[allow(dead_code)]
    pub const fn long(&self) -> &String {
        &self.long
    }
}

#[derive(Default)]
pub struct FlagNormalizer {
    inner: std::collections::HashMap<char, String>,
}

impl FlagNormalizer {
    /// short フラグと long フラグの対応を追加する
    #[allow(dead_code)]
    pub fn try_add(&mut self, short: &char, long: &str) -> Result<(), FlagError> {
        if let Some(dup_long) = self.inner.get(short) {
            return Err(FlagError::DuplicateShortFlag(
                *short,
                long.to_owned(),
                dup_long.to_owned(),
            ));
        }
        if self.inner.insert(*short, long.to_owned()).is_some() {
            panic!("Illigal inner state");
        }
        Ok(())
    }

    /// short フラグをもとに正規化されたフラグを得る
    #[allow(dead_code)]
    pub fn try_get(&self, short: &char) -> Option<NormalizedFlag> {
        let long = self.inner.get(short)?;
        Some(NormalizedFlag {
            short: Some(*short),
            long: long.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FlagNormalizer;

    #[test]
    fn test_try_add() {
        let mut normalizer = FlagNormalizer::default();
        assert!(normalizer.try_add(&'o', "output").is_ok());
        assert!(normalizer.try_add(&'v', "version").is_ok());
        assert!(normalizer.try_add(&'o', "output").is_err());
    }

    #[test]
    fn test_try_get() {
        let mut normalizer = FlagNormalizer::default();
        assert!(normalizer.try_add(&'o', "output").is_ok());
        assert!(matches!(
            normalizer.try_get(&'o'),
            Some(flag)
                if flag.long() == "output" && matches!(flag.short(), Some(c) if *c == 'o')
        ));
    }
}
