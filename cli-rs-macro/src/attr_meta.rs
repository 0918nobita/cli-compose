pub fn extract_meta<'a, A>(attrs: A, name: &'a str) -> impl Iterator<Item = syn::NestedMeta> + 'a
where
    A: Iterator<Item = &'a syn::Attribute> + 'a,
{
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
