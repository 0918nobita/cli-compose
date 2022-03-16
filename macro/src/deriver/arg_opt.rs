use bae::FromAttributes;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

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

fn derive_parse_fn_from_enum(ty_name: &syn::Ident) -> TokenStream {
    quote! {
        fn parse(s: &str) -> Option<Self> {
            <#ty_name as std::str::FromStr>::from_str(s).ok()
        }
    }
}

fn derive_parse_fn_from_struct(
    struct_token: &syn::token::Struct,
    fields: &syn::Fields,
    ty_name: &syn::Ident,
) -> syn::Result<TokenStream> {
    let field = match fields.iter().collect::<Vec<_>>()[..] {
        [field] => field,
        _ => return Err(syn::Error::new_spanned(struct_token, UNSUPPORTED_SHAPE)),
    };

    let ty = &field.ty;

    Ok(quote! {
        fn parse(s: &str) -> Option<Self> {
            let val = <#ty as std::str::FromStr>::from_str(s).ok()?;
            Some(#ty_name(val))
        }
    })
}

pub fn derive_arg_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let attr = ArgOpt::try_from_attributes(&input.attrs)?;

    let ty_name = &input.ident;

    let long = attr
        .as_ref()
        .and_then(|arg_opt| arg_opt.long.clone())
        .map_or_else(
            || input.ident.to_string().to_case(Case::Kebab),
            |lit_str| lit_str.value(),
        );

    let flag = match attr.as_ref().and_then(|arg_opt| arg_opt.short.clone()) {
        Some(short) => {
            quote! { cli_compose::schema::Flag::BothLongAndShort(#long.to_owned(), #short) }
        }
        None => quote! { cli_compose::schema::Flag::LongOnly(#long.to_owned()) },
    };

    let doc = extract_doc(&input.attrs);

    let parse_fn = match &input.data {
        Data::Enum(_) => Ok(derive_parse_fn_from_enum(ty_name)),

        Data::Struct(syn::DataStruct {
            struct_token,
            fields,
            ..
        }) => derive_parse_fn_from_struct(struct_token, fields, ty_name),

        Data::Union(data_union) => Err(syn::Error::new_spanned(
            data_union.union_token,
            UNSUPPORTED_SHAPE,
        )),
    }?;

    let sharp = syn::Token![#](proc_macro2::Span::call_site());

    Ok(quote! {
        impl cli_compose::schema::AsMember for #ty_name {
            fn handle(mut builder: cli_compose::schema::CliBuilder) -> cli_compose::schema::CliBuilder {
                use cli_compose::schema::{forwarded::quote::quote, AsArgOpt};

                let flag = format!("{}", <#ty_name as AsArgOpt>::flag());

                builder.ops.extend(quote! {
                    println!("   Opt {}", #sharp flag);
                });

                builder
            }
        }

        impl cli_compose::schema::AsArgOpt for #ty_name {
            fn flag() -> cli_compose::schema::Flag {
                #flag
            }

            fn description() -> String {
                #doc.to_owned()
            }

            #parse_fn
        }
    })
}

#[cfg(test)]
mod tests {
    use quote::quote;

    fn test_derive_arg_opt(input: proc_macro2::TokenStream) -> anyhow::Result<String> {
        let tokens = super::derive_arg_opt(input)?;

        crate::pretty_print::pretty_print_rust_code(tokens)
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

    #[test]
    fn _enum() {
        insta::assert_display_snapshot!(test_derive_arg_opt(quote! {
            enum Foo { Bar, Baz }
        })
        .unwrap());
    }

    #[test]
    fn _union() {
        insta::assert_debug_snapshot!(test_derive_arg_opt(quote! {
            union Foo { f1: i32, f2: u32 }
        }));
    }
}
