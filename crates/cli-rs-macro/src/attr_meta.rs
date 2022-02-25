pub fn extract_meta<'a>(
    attrs: impl Iterator<Item = &'a syn::Attribute> + 'a,
    name: &'a str,
) -> impl Iterator<Item = syn::NestedMeta> + 'a {
    attrs
        .filter_map(|attr| {
            attr.parse_meta().ok().and_then(|meta| match meta {
                syn::Meta::List(syn::MetaList { path, nested, .. }) if path.is_ident(name) => {
                    Some(nested)
                }
                _ => None,
            })
        })
        .flatten()
}
