use bae::FromAttributes;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use crate::doc::extract_doc;

static UNSUPPORTED_SHAPE: &str =
    "#[derive(ArgOpt)] can only be applied to structs with single unnamed field or enums";

#[derive(FromAttributes)]
struct ArgOpt {
    long: Option<syn::LitStr>,

    short: Option<syn::LitChar>,

    #[allow(dead_code)]
    short_only: Option<()>,

    #[allow(dead_code)]
    use_default: Option<()>,
}

pub fn derive_arg_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let attr = ArgOpt::try_from_attributes(&input.attrs)?;

    let ty_name = &input.ident;

    match &input.data {
        syn::Data::Enum(_) => {
            let long = attr
                .as_ref()
                .and_then(|arg_opt| arg_opt.long.clone())
                .map_or_else(
                    || input.ident.to_string().to_case(Case::Kebab),
                    |lit_str| lit_str.value(),
                );

            let flag = match attr.and_then(|arg_opt| arg_opt.short) {
                Some(short) => {
                    quote! { cli_compose::schema::Flag::BothLongAndShort(#long.to_owned(), #short) }
                }
                None => quote! { cli_compose::schema::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(&input.attrs);

            Ok(quote! {
                impl cli_compose::schema::AsArgOpt for #ty_name {
                    fn flag() -> cli_compose::schema::Flag {
                        #flag
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        <#ty_name as std::str::FromStr>::from_str(s).ok()
                    }
                }
            })
        }

        syn::Data::Struct(syn::DataStruct {
            struct_token,
            fields,
            ..
        }) => {
            let field = match fields.iter().collect::<Vec<_>>()[..] {
                [field] => field,
                _ => return Err(syn::Error::new_spanned(struct_token, UNSUPPORTED_SHAPE)),
            };

            let long = attr
                .as_ref()
                .and_then(|arg_opt| arg_opt.long.clone())
                .map_or_else(
                    || ty_name.to_string().to_case(Case::Kebab),
                    |lit_str| lit_str.value(),
                );

            let flag = match attr.and_then(|arg_opt| arg_opt.short) {
                Some(short) => {
                    quote! { cli_compose::schema::Flag::BothLongAndShort(#long.to_owned(), #short) }
                }
                None => quote! { cli_compose::schema::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(&input.attrs);

            let ty = field.ty.clone();

            Ok(quote! {
                impl cli_compose::schema::AsArgOpt for #ty_name {
                    fn flag() -> cli_compose::schema::Flag {
                        #flag
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        let val = <#ty as std::str::FromStr>::from_str(s).ok()?;
                        Some(#ty_name(val))
                    }
                }
            })
        }

        syn::Data::Union(data_union) => Err(syn::Error::new_spanned(
            data_union.union_token,
            UNSUPPORTED_SHAPE,
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process;

    use quote::quote;

    fn test_derive_arg_opt(input: proc_macro2::TokenStream) -> anyhow::Result<String> {
        let tokens = super::derive_arg_opt(input)?;

        let mut rustfmt = process::Command::new("rustfmt")
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()?;

        write!(rustfmt.stdin.take().unwrap(), "{}", tokens)?;

        let output = rustfmt.wait_with_output()?;

        let stdout = String::from_utf8(output.stdout)?;

        Ok(stdout)
    }

    #[test]
    fn empty() {
        insta::assert_debug_snapshot!(test_derive_arg_opt(quote! {}));
    }

    #[test]
    fn struct_without_field() {
        insta::assert_debug_snapshot!(test_derive_arg_opt(quote! {
            struct Foo;
        }));
    }

    #[test]
    fn struct_with_single_field() {
        insta::assert_display_snapshot!(test_derive_arg_opt(quote! {
            struct Foo(String);
        })
        .unwrap());
    }

    #[test]
    fn struct_with_multiple_fields() {
        insta::assert_debug_snapshot!(test_derive_arg_opt(quote! {
            struct Foo(String, i32);
        }));
    }
}
