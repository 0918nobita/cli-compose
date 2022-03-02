use proc_macro2::TokenStream;

pub fn derive_from_kebab_str(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;
    let ty_name = &derive_input.ident;
    let _data_enum = match &derive_input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return Err(syn::Error::new_spanned(
                derive_input,
                "`#[derive(FromKebabStr)]` can only be applied to enums",
            ));
        }
    };
    Ok(quote::quote! {
        impl std::str::FromStr for #ty_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                unimplemented!()
            }
        }
    })
}
