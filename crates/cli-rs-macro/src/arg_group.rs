use proc_macro::TokenStream;
use quote::quote;

pub fn derive_arg_group(_: TokenStream) -> TokenStream {
    quote! {}.into()
}
