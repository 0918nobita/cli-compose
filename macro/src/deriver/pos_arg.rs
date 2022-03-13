use bae::FromAttributes;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use crate::doc::extract_doc;

#[derive(FromAttributes)]
struct PosArg {
    name: Option<syn::LitStr>,

    #[allow(dead_code)]
    use_default: Option<()>,
}

pub fn derive_pos_arg(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let attr = PosArg::try_from_attributes(&input.attrs)?;

    let doc = extract_doc(&input.attrs);

    let struct_name = &input.ident;
    let struct_name_kebab_case = attr
        .and_then(|attr| attr.name)
        .map(|lit_str| lit_str.value())
        .unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

    let parse_method = match &input.data {
        syn::Data::Enum(_) => {
            quote! {
                fn parse(s: &str) -> Option<Self> {
                    <#struct_name as std::str::FromStr>::from_str(s).ok()
                }
            }
        }

        syn::Data::Struct(data_struct) => match data_struct.fields.iter().collect::<Vec<_>>()[..] {
            [field] => {
                let ty = &field.ty;
                if let Some(ident) = &field.ident {
                    quote! {
                        fn parse(s: &str) -> Option<Self> {
                            <#ty as std::str::FromStr>::from_str(s).ok().map(|v| Self { #ident: v })
                        }
                    }
                } else {
                    quote! {
                        fn parse(s: &str) -> Option<Self> {
                            <#ty as std::str::FromStr>::from_str(s).ok().map(Self)
                        }
                    }
                }
            }
            [] => return Err(syn::Error::new_spanned(input.ident, "missing field")),
            _ => {
                return Err(syn::Error::new_spanned(
                    data_struct.struct_token,
                    "multiple fields are not allowed",
                ))
            }
        },

        syn::Data::Union(data_union) => {
            return Err(syn::Error::new_spanned(
                data_union.union_token,
                "unions are not allowed",
            ));
        }
    };

    Ok(quote::quote! {
        impl cli_compose::schema::AsPosArg for #struct_name {
            fn name() -> String {
                #struct_name_kebab_case.to_owned()
            }

            fn description() -> String {
                #doc.to_owned()
            }

            #parse_method
        }
    })
}

#[cfg(test)]
mod tests {
    use quote::quote;

    fn test_derive_pos_arg(input: proc_macro2::TokenStream) -> anyhow::Result<String> {
        let tokens = super::derive_pos_arg(input)?;

        crate::pretty_print::pretty_print_rust_code(tokens)
    }

    #[test]
    fn empty() {
        insta::assert_debug_snapshot!(test_derive_pos_arg(quote! {}));
    }

    #[test]
    fn struct_without_field() {
        insta::assert_debug_snapshot!(test_derive_pos_arg(quote! {
            struct Foo;
        }));
    }

    #[test]
    fn struct_with_single_field() {
        insta::assert_display_snapshot!(test_derive_pos_arg(quote! {
            struct Foo(String);
        })
        .unwrap());
    }

    #[test]
    fn struct_with_multiple_fields() {
        insta::assert_debug_snapshot!(test_derive_pos_arg(quote! {
            struct Foo(String, i32);
        }));
    }
}
