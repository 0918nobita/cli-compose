use syn::Attribute;

fn try_get_single_line_doc(attr: Attribute) -> Option<String> {
    let tokens = match attr {
        Attribute {
            path,
            style: syn::AttrStyle::Outer,
            tokens,
            ..
        } if path.is_ident("doc") => tokens,
        _ => return None,
    };
    let doc = tokens
        .into_iter()
        .skip(1)
        .collect::<proc_macro2::TokenStream>();
    Some(format!("{}", doc).trim_matches('"').trim_start().to_owned())
}

pub fn extract_doc(attrs: impl Iterator<Item = Attribute>) -> String {
    attrs
        .filter_map(try_get_single_line_doc)
        .collect::<Vec<_>>()
        .join("\n")
}
