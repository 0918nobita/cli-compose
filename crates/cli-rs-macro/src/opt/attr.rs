use quote::ToTokens;

use super::result::{OptErr, OptErrKind, OptResult};
use crate::attr_meta::extract_meta;

#[derive(Default)]
pub struct OptAttr {
    pub long: Option<String>,
    pub short: Option<char>,
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
            _ => {
                return Err(OptErr::new(
                    OptErrKind::UnexpectedLit,
                    nested_meta.to_token_stream(),
                ))
            }
        };

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,
            _ => return Err(OptErr::new(OptErrKind::InvalidMeta, meta.to_token_stream())),
        };

        if path.is_ident("long") {
            let lit = match lit {
                syn::Lit::Str(lit) => lit,
                _ => {
                    return Err(OptErr::new(
                        OptErrKind::InvalidLongValue,
                        lit.to_token_stream(),
                    ))
                }
            };
            attr.long = Some(lit.value());
        } else if path.is_ident("short") {
            let lit = match lit {
                syn::Lit::Char(lit) => lit,
                _ => {
                    return Err(OptErr::new(
                        OptErrKind::InvalidShortValue,
                        lit.to_token_stream(),
                    ))
                }
            };
            attr.short = Some(lit.value());
        } else {
            return Err(OptErr::new(
                OptErrKind::UnexpectedKey,
                path.to_token_stream(),
            ));
        }
    }

    Ok(attr)
}
