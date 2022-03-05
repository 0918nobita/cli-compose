use convert_case::{Case, Casing};
use darling::{ast::Data, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromDeriveInput)]
struct FromKebabStr {
    ident: syn::Ident,

    data: Data<syn::Ident, syn::Field>,
}

pub fn derive_from_kebab_str(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let input = match FromKebabStr::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => return Ok(err.write_errors()),
    };

    let ty_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(variants) => Ok(variants),
        Data::Struct(fields) => Err(syn::Error::new_spanned(
            &fields,
            "`#[derive(FromKebabStr)]` can only be applied to enums whose variants have no field",
        )),
    }?;

    let arms = variants
        .iter()
        .map(|variant| {
            let variant_str = variant.to_string().to_case(Case::Kebab);
            quote! { #variant_str => Ok(#ty_name::#variant), }
        })
        .collect::<TokenStream>();

    Ok(quote! {
        impl std::str::FromStr for #ty_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #arms
                    _ => Err(()),
                }
            }
        }
    })
}
