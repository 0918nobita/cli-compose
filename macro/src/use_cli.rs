use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use std::path::MAIN_SEPARATOR;

pub fn use_cli(input: TokenStream) -> syn::Result<TokenStream> {
    let path = syn::parse2::<syn::Path>(input)?;

    let ident = &path
        .segments
        .iter()
        .last()
        .expect("Failed to extract the last path segment")
        .ident;

    let source_path = format!(
        "{sep}cli_compose{sep}{filename}.rs",
        sep = MAIN_SEPARATOR,
        filename = ident.to_string().to_case(Case::Kebab),
    );

    Ok(quote::quote! {
        include!(concat!(env!("OUT_DIR"), #source_path));
        use #path;
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_use_cli() {
        let input = quote::quote! { opts::ExampleOpts };
        insta::assert_display_snapshot!(super::use_cli(input)
            .map(|tokens| crate::pretty_print::pretty_print_rust_code(tokens).unwrap())
            .unwrap());
    }
}
