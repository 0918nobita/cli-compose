use derive_more::Display;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Data, NestedMeta};

use crate::{attr_meta::extract_meta, doc::extract_doc, kebab_case::upper_camel_to_kebab};

#[derive(Debug, Display)]
#[display(fmt = "#[derive(Opt)] can only be applied to empty structs")]
struct InvalidStruct;

fn validate_struct(data: &Data) -> Result<(), InvalidStruct> {
    match data {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => Ok(()),
        _ => Err(InvalidStruct),
    }
}

#[derive(Default)]
struct OptAttr {
    long: Option<String>,
    short: Option<char>,
}

// HACK: 可読性を上げたい
fn extract_opt_attr<'a>(attrs: impl Iterator<Item = &'a Attribute> + 'a) -> syn::Result<OptAttr> {
    let mut attr = OptAttr::default();

    for nested_meta in extract_meta(attrs, "opt") {
        let meta = match nested_meta {
            NestedMeta::Meta(meta) => meta,
            _ => {
                return Err(syn::Error::new_spanned(
                    nested_meta,
                    "Literals in #[opt(..)] are not allowed",
                ))
            }
        };

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,
            _ => {
                return Err(syn::Error::new_spanned(
                    meta,
                    "Metadata in #[opt(..)] is invalid",
                ))
            }
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => lit,
                _ => {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "#[opt(long = ..)] must be a string literal",
                    ))
                }
            };
            attr.long = Some(lit.value());
        } else if path.is_ident("short") {
            let lit = match lit {
                syn::Lit::Char(lit) => lit,
                _ => {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "#[opt(short = ..)] must be a char literal",
                    ))
                }
            };
            attr.short = Some(lit.value());
        } else {
            return Err(syn::Error::new_spanned(
                path,
                "Unexpected key in #[opt(..)]",
            ));
        }
    }

    Ok(attr)
}

pub fn derive_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    validate_struct(&derive_input.data)
        .map_err(|err| syn::Error::new_spanned(&derive_input, err.to_string()))?;

    let OptAttr { long, short } = extract_opt_attr(derive_input.attrs.iter())?;
    let short = match short {
        Some(lit) => quote! { Some(cli_rs::ShortFlag::new(#lit)) },
        None => quote! { None },
    };

    let doc = extract_doc(derive_input.attrs.iter());

    let struct_name = derive_input.ident;
    let struct_name_kebab_case =
        long.unwrap_or_else(|| upper_camel_to_kebab(&struct_name.to_string()));

    Ok(quote! {
        impl cli_rs::AsOpt for #struct_name {
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

#[derive(Debug, Display)]
pub enum FlagError {
    #[display(
        fmt = "Duplicate short flag: `{} -> {}` and `{} -> {}`",
        _0,
        _1,
        _0,
        _2
    )]
    DuplicateShortFlag(char, String, String),
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
