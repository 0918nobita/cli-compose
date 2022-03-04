use quote::ToTokens;

use super::result::{ArgOptErr, ArgOptErrKind, ArgOptResult};
use crate::attr_meta::extract_meta;

#[derive(Default)]
pub struct ArgOptAttr {
    pub long: Option<String>,
    pub short: Option<char>,
}

macro_rules! err {
    ($kind:ident, $to_tokens:expr) => {
        ArgOptErr::new(ArgOptErrKind::$kind, $to_tokens.to_token_stream())
    };
}

pub fn extract_arg_opt_attr<'a, A>(attrs: A) -> ArgOptResult<ArgOptAttr>
where
    A: Iterator<Item = &'a syn::Attribute> + 'a,
{
    let mut attr = ArgOptAttr::default();

    for nested_meta in extract_meta(attrs, "arg_opt") {
        let meta = match nested_meta {
            syn::NestedMeta::Meta(meta) => Ok(meta),
            _ => Err(err!(UnexpectedLit, nested_meta)),
        }?;

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,

            // TODO: default を指定した場合のコード生成を実装する
            syn::Meta::Path(path) if path.is_ident("default") => {
                continue;
            }

            _ => return Err(err!(InvalidMeta, meta)),
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => Ok(lit),
                _ => Err(err!(InvalidLongValue, lit)),
            }?;
            attr.long = Some(lit.value());
        } else if path.is_ident("short") {
            let lit = match lit {
                syn::Lit::Char(lit) => Ok(lit),
                _ => Err(err!(InvalidShortValue, lit)),
            }?;
            attr.short = Some(lit.value());
        } else {
            return Err(err!(UnexpectedKey, path));
        }
    }

    Ok(attr)
}
