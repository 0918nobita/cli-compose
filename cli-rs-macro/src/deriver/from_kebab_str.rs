use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

static INVALID_DERIVE_INPUT: &str =
    "`#[derive(FromKebabStr)]` can only be applied to enums whose variants have no field";

pub fn derive_from_kebab_str(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    let ty_name = &derive_input.ident;

    let data_enum = match &derive_input.data {
        syn::Data::Enum(data_enum) => Ok(data_enum),
        _ => Err(syn::Error::new_spanned(&derive_input, INVALID_DERIVE_INPUT)),
    }?;

    let mut variants = Vec::<syn::Ident>::new();

    for variant in &data_enum.variants {
        match &variant.fields {
            syn::Fields::Unit => {}
            _ => return Err(syn::Error::new_spanned(&variant, INVALID_DERIVE_INPUT)),
        }

        variants.push(variant.ident.clone());
    }

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
