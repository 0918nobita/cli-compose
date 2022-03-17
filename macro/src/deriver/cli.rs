use proc_macro2::TokenStream;

pub fn derive_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let input: syn::DeriveInput = syn::parse2(input)?;

    let ident = &input.ident;
    let ident_str = ident.to_string();

    Ok(quote::quote! {
        impl cli_compose::schema::AsCliMeta for #ident {
            fn ident() -> cli_compose::schema::forwarded::syn::Ident {
                cli_compose::schema::ident(#ident_str)
            }
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_derive_cli() {
        let input = quote::quote! { struct Cli; };
        insta::assert_display_snapshot!(super::derive_cli(input)
            .map(|tokens| crate::pretty_print::pretty_print_rust_code(tokens).unwrap())
            .unwrap());
    }
}
