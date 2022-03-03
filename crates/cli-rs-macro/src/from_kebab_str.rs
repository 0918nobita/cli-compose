use proc_macro2::TokenStream;

static INVALID_DERIVE_INPUT: &str =
    "`#[derive(FromKebabStr)]` can only be applied to enums whose variants have no field";

pub fn derive_from_kebab_str(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;
    let ty_name = &derive_input.ident;
    let data_enum = match &derive_input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return Err(syn::Error::new_spanned(derive_input, INVALID_DERIVE_INPUT));
        }
    };
    // TODO: 取り出した各識別子をもとに、from_str メソッドの定義コードを生成する
    for variant in &data_enum.variants {
        match &variant.fields {
            syn::Fields::Unit => {}
            _ => return Err(syn::Error::new_spanned(variant, INVALID_DERIVE_INPUT)),
        }
        let _ident = variant.ident.clone();
    }
    Ok(quote::quote! {
        impl std::str::FromStr for #ty_name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                unimplemented!()
            }
        }
    })
}
