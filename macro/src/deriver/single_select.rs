use proc_macro2::TokenStream;

pub fn derive_single_select(_: TokenStream) -> syn::Result<TokenStream> {
    Ok(TokenStream::new())
}
