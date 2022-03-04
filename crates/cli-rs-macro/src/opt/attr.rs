use quote::ToTokens;

use super::result::{OptErr, OptErrKind, OptResult};
use crate::attr_meta::extract_meta;

#[derive(Default)]
pub struct OptAttr {
    pub long: Option<String>,
    pub short: Option<char>,
}

macro_rules! err {
    ($kind:ident, $to_tokens:expr) => {
        OptErr::new(OptErrKind::$kind, $to_tokens.to_token_stream())
    };
}

// HACK: 可読性を上げたい
pub fn extract_opt_attr<'a, A>(attrs: A) -> OptResult<OptAttr>
where
    A: Iterator<Item = &'a syn::Attribute> + 'a,
{
    let mut attr = OptAttr::default();

    for nested_meta in extract_meta(attrs, "opt") {
        let meta = match nested_meta {
            syn::NestedMeta::Meta(meta) => meta,
            _ => return Err(err!(UnexpectedLit, nested_meta)),
        };

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,
            _ => return Err(err!(InvalidMeta, meta)),
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => lit,
                _ => return Err(err!(InvalidLongValue, lit)),
            };
            attr.long = Some(lit.value());
        } else if path.is_ident("short") {
            let lit = match lit {
                syn::Lit::Char(lit) => lit,
                _ => return Err(err!(InvalidShortValue, lit)),
            };
            attr.short = Some(lit.value());
        } else {
            return Err(err!(UnexpectedKey, path));
        }
    }

    Ok(attr)
}
