use proc_macro2::TokenStream;

pub fn derive_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let input: syn::DeriveInput = syn::parse2(input)?;

    let ident = &input.ident;
    let ident_str = ident.to_string();

    Ok(quote::quote! {
        impl cli_compose::schema::AsCliMeta for #ident {
            fn ident() -> cli_compose::schema::Ident {
                cli_compose::schema::ident(#ident_str)
            }
        }
    })
}
