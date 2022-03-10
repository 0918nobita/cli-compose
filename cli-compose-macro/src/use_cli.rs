use proc_macro2::TokenStream;

pub fn use_cli(_input: TokenStream) -> syn::Result<TokenStream> {
    Ok(TokenStream::new())
}
