use quote::ToTokens;

use super::result::{PosArgErr, PosArgErrKind, PosArgResult};
use crate::attr_meta::extract_meta;

#[derive(Default)]
pub struct PosArgAttr {
    pub name: Option<String>,
}

macro_rules! err {
    ($kind:ident, $to_tokens:expr) => {
        PosArgErr::new(PosArgErrKind::$kind, $to_tokens.to_token_stream())
    };
}

pub fn extract_pos_arg_attr<'a, A>(attrs: A) -> PosArgResult<PosArgAttr>
where
    A: Iterator<Item = &'a syn::Attribute> + 'a,
{
    let mut attr = PosArgAttr::default();

    for nested_meta in extract_meta(attrs, "arg") {
        let meta = match nested_meta {
            syn::NestedMeta::Meta(meta) => Ok(meta),
            _ => Err(err!(UnexpectedLit, nested_meta)),
        }?;

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => Ok(name_value),
            _ => Err(err!(InvalidMeta, meta)),
        }?;

        if path.is_ident("name") {
            let lit = match lit {
                syn::Lit::Str(lit) => Ok(lit),
                _ => Err(err!(InvalidNameValue, lit)),
            }?;
            attr.name = Some(lit.value());
        } else {
            return Err(err!(UnexpectedKey, path));
        }
    }

    Ok(attr)
}
