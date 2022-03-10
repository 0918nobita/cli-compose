use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use std::path::MAIN_SEPARATOR;

pub fn use_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let ident = syn::parse2::<syn::Ident>(input)?;
    let path = format!(
        "{}cli_compose{}{}.rs",
        MAIN_SEPARATOR,
        MAIN_SEPARATOR,
        ident.to_string().to_case(Case::Kebab)
    );
    Ok(quote::quote! {
        include!(concat!(env!("OUT_DIR"), #path));
    })
}
