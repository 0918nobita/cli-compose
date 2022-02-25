use syn::Attribute;

fn try_get_single_line_doc(attr: Attribute) -> Option<String> {
    let meta = attr.parse_meta().ok()?;
    match meta {
        syn::Meta::NameValue(syn::MetaNameValue {
            path,
            lit: syn::Lit::Str(lit_str),
            ..
        }) if path.is_ident("doc") => Some(lit_str.value().trim_start().to_owned()),
        _ => None,
    }
}

pub fn extract_doc(attrs: impl Iterator<Item = Attribute>) -> String {
    attrs
        .filter_map(try_get_single_line_doc)
        .collect::<Vec<_>>()
        .join("\n")
}
