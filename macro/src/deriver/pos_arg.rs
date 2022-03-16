use bae::FromAttributes;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Data;

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

    let ty_name = &input.ident;
    let ty_name_str = ty_name.to_string();
    let ty_name_kebab_case = attr.and_then(|attr| attr.name).map_or_else(
        || ty_name_str.to_case(Case::Kebab),
        |lit_str| lit_str.value(),
    );

    let methods = match &input.data {
        Data::Enum(_) => {
            quote! {
                fn parse(s: &str) -> Option<Self> {
                    <#ty_name as std::str::FromStr>::from_str(s).ok()
                }

                fn result() -> cli_compose::schema::forwarded::syn::Type {
                    cli_compose::schema::forwarded::syn::parse_str(#ty_name_str).unwrap()
                }
            }
        }

        Data::Struct(data_struct) => match data_struct.fields.iter().collect::<Vec<_>>()[..] {
            [field] => {
                let ty = &field.ty;
                let ty_name = ty.into_token_stream().to_string();
                let parse_method = if let Some(ident) = &field.ident {
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
                };
                quote! {
                    #parse_method

                    fn result() -> cli_compose::schema::forwarded::syn::Type {
                        cli_compose::schema::forwarded::syn::parse_str(#ty_name).unwrap()
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

        Data::Union(data_union) => {
            return Err(syn::Error::new_spanned(
                data_union.union_token,
                "unions are not allowed",
            ));
        }
    };

    let sharp = syn::Token![#](proc_macro2::Span::call_site());

    Ok(quote::quote! {
        impl cli_compose::schema::AsMember for #ty_name {
            fn handle(mut builder: cli_compose::schema::CliBuilder) -> cli_compose::schema::CliBuilder {
                use cli_compose::schema::{forwarded::{syn, quote}, AsPosArg};

                let name = <#ty_name as AsPosArg>::name();

                let res_ty =
                    <syn::Type as quote::ToTokens>::into_token_stream(
                        <#ty_name as AsPosArg>::result()
                    ).to_string();

                builder.ops.extend(quote::quote! {
                    println!("PosArg {} ({})", #sharp name, #sharp res_ty);
                });

                builder
            }
        }

        impl cli_compose::schema::AsPosArg for #ty_name {
            fn name() -> String {
                #ty_name_kebab_case.to_owned()
            }

            fn description() -> String {
                #doc.to_owned()
            }

            #methods
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
