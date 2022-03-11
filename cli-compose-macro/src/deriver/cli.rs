use proc_macro2::TokenStream;

pub fn derive_cli(_: TokenStream) -> syn::Result<TokenStream> {
    Ok(TokenStream::new())
}
