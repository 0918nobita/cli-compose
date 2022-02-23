use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Arg)]
pub fn derive_arg(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let struct_name = item.ident;
    quote! {
        impl #struct_name {
            fn answer() -> u32 { 42 }
        }
    }
    .into()
}
