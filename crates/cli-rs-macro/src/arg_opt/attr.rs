use quote::ToTokens;

use super::result::{ArgOptError, ArgOptErrorKind, ArgOptResult};
use crate::attr_meta::extract_meta;

#[derive(Default)]
pub struct ArgOptAttr {
    pub long: Option<String>,
    pub short: Option<char>,
}

// HACK: 可読性を上げたい
pub fn extract_arg_opt_attr<'a, A>(attrs: A) -> ArgOptResult<ArgOptAttr>
where
    A: Iterator<Item = &'a syn::Attribute> + 'a,
{
    let mut attr = ArgOptAttr::default();

    for nested_meta in extract_meta(attrs, "arg_opt") {
        let meta = match nested_meta {
            syn::NestedMeta::Meta(meta) => Ok(meta),
            _ => Err(ArgOptError::new(
                ArgOptErrorKind::UnexpectedLit,
                nested_meta.to_token_stream(),
            )),
        }?;

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,

            // TODO: default を指定した場合のコード生成を実装する
            syn::Meta::Path(path) if path.is_ident("default") => {
                continue;
            }

            _ => {
                return Err(ArgOptError::new(
                    ArgOptErrorKind::InvalidMeta,
                    meta.to_token_stream(),
                ))
            }
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => Ok(lit),
                _ => Err(ArgOptError::new(
                    ArgOptErrorKind::InvalidLongValue,
                    lit.to_token_stream(),
                )),
            }?;
            attr.long = Some(lit.value());
        } else if path.is_ident("short") {
            let lit = match lit {
                syn::Lit::Char(lit) => Ok(lit),
                _ => Err(ArgOptError::new(
                    ArgOptErrorKind::InvalidShortValue,
                    lit.to_token_stream(),
                )),
            }?;
            attr.short = Some(lit.value());
        } else {
            return Err(ArgOptError::new(
                ArgOptErrorKind::UnexpectedKey,
                path.to_token_stream(),
            ));
        }
    }

    Ok(attr)
}
