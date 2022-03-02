use proc_macro2::TokenStream;

pub fn derive_group(_: TokenStream) -> syn::Result<TokenStream> {
    Ok(quote::quote! {})
}
