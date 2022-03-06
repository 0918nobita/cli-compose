use syn::{Attribute, Lit, Meta};

fn try_get_single_line_doc(attr: &Attribute) -> Option<String> {
    let meta = attr.parse_meta().ok()?;

    let lit_str = match meta {
        Meta::NameValue(syn::MetaNameValue {
            path,
            lit: Lit::Str(lit_str),
            ..
        }) if path.is_ident("doc") => lit_str,
        _ => return None,
    };

    Some(lit_str.value().trim_start().to_owned())
}

pub fn extract_doc(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter_map(try_get_single_line_doc)
        .collect::<Vec<_>>()
        .join("\n")
}
