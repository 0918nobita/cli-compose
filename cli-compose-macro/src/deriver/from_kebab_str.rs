use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

static UNSUPPORTED_SHAPE: &str =
    "`#[derive(FromKebabStr)]` can only be applied to enums whose variants have no field";

pub fn derive_from_kebab_str(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let ty_name = &input.ident;

    let data_enum = match &input.data {
        syn::Data::Enum(data_enum) => Ok(data_enum),
        syn::Data::Struct(data_struct) => Err(syn::Error::new_spanned(
            data_struct.struct_token,
            UNSUPPORTED_SHAPE,
        )),
        syn::Data::Union(data_union) => Err(syn::Error::new_spanned(
            data_union.union_token,
            UNSUPPORTED_SHAPE,
        )),
    }?;

    let arms = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_str = variant.ident.to_string().to_case(Case::Kebab);
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
