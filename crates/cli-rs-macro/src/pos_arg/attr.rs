use quote::ToTokens;

use super::result::{PosArgErr, PosArgErrKind, PosArgResult};
use crate::attr_meta::extract_meta;

#[derive(Default)]
pub struct PosArgAttr {
    pub name: Option<String>,
}

pub fn extract_pos_arg_attr<'a, A>(attrs: A) -> PosArgResult<PosArgAttr>
where
    A: Iterator<Item = &'a syn::Attribute> + 'a,
{
    let mut attr = PosArgAttr::default();

    for nested_meta in extract_meta(attrs, "arg") {
        let meta = match nested_meta {
            syn::NestedMeta::Meta(meta) => Ok(meta),
            _ => Err(PosArgErr::new(
                PosArgErrKind::UnexpectedLit,
                nested_meta.to_token_stream(),
            )),
        }?;

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => Ok(name_value),
            _ => Err(PosArgErr::new(
                PosArgErrKind::InvalidMeta,
                meta.to_token_stream(),
            )),
        }?;

        if path.is_ident("name") {
            let lit = match lit {
                syn::Lit::Str(lit) => Ok(lit),
                _ => Err(PosArgErr::new(
                    PosArgErrKind::InvalidNameValue,
                    lit.to_token_stream(),
                )),
            }?;
            attr.name = Some(lit.value());
        } else {
            return Err(PosArgErr::new(
                PosArgErrKind::UnexpectedKey,
                path.to_token_stream(),
            ));
        }
    }

    Ok(attr)
}
