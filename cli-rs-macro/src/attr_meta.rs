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

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse::Parser;

    use super::extract_meta;

    fn extract_meta_str<'a, A>(attrs: A, name: &'a str) -> Vec<String>
    where
        A: Iterator<Item = &'a syn::Attribute> + 'a,
    {
        extract_meta(attrs, name)
            .map(|nested_meta| nested_meta.to_token_stream().to_string())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_extract_meta() -> syn::Result<()> {
        let attrs = syn::Attribute::parse_inner.parse2(quote::quote! {
            #![foo(num = 42)]
            #![bar(str = "hello")]
            #![baz(path)]
        })?;

        let foo = extract_meta_str(attrs.iter(), "foo");
        let bar = extract_meta_str(attrs.iter(), "bar");
        let baz = extract_meta_str(attrs.iter(), "baz");

        insta::assert_debug_snapshot!(vec![foo, bar, baz]);
        Ok(())
    }
}
