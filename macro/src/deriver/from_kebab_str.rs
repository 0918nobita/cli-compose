use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

static UNSUPPORTED_SHAPE: &str =
    "`#[derive(FromKebabStr)]` can only be applied to enums whose variants have no field";

pub fn derive_from_kebab_str(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let ty_name = &input.ident;

    let data_enum = match &input.data {
        Data::Enum(data_enum) => Ok(data_enum),
        Data::Struct(data_struct) => Err(syn::Error::new_spanned(
            data_struct.struct_token,
            UNSUPPORTED_SHAPE,
        )),
        Data::Union(data_union) => Err(syn::Error::new_spanned(
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

#[cfg(test)]
mod tests {
    use quote::quote;

    fn test_from_kebab_str_deriver(input: proc_macro2::TokenStream) -> anyhow::Result<String> {
        let tokens = super::derive_from_kebab_str(input)?;

        crate::pretty_print::pretty_print_rust_code(tokens)
    }

    #[test]
    fn empty() {
        insta::assert_debug_snapshot!(test_from_kebab_str_deriver(quote! {}));
    }

    #[test]
    fn _enum() {
        let input = quote! {
            enum TextFileFormat {
                Json,
                Yaml,
                ReStructuredText,
            }
        };
        insta::assert_display_snapshot!(test_from_kebab_str_deriver(input).unwrap());
    }

    #[test]
    fn _struct() {
        let input = quote! {
            struct Foo;
        };
        insta::assert_debug_snapshot!(test_from_kebab_str_deriver(input));
    }

    #[test]
    fn _union() {
        insta::assert_debug_snapshot!(test_from_kebab_str_deriver(quote! {
            union Foo { f1: i32, f2: u32 }
        }));
    }
}
