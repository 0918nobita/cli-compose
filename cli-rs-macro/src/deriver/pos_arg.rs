mod result;

use convert_case::{Case, Casing};
use darling::FromDeriveInput;
use proc_macro2::TokenStream;

use crate::doc::extract_doc;

#[derive(FromDeriveInput)]
#[darling(attributes(pos_arg), forward_attrs(doc))]
struct PosArgInput {
    ident: syn::Ident,

    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    name: Option<String>,
}

pub fn derive_pos_arg(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let input = match PosArgInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => return Ok(err.write_errors()),
    };

    let doc = extract_doc(input.attrs.iter());

    let struct_name = &input.ident;
    let struct_name_kebab_case = input
        .name
        .unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

    Ok(quote::quote! {
        impl cli_rs::AsPosArg for #struct_name {
            fn name() -> String {
                #struct_name_kebab_case.to_owned()
            }

            fn description() -> String {
                #doc.to_owned()
            }
        }
    })
}
