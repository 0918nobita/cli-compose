use bae::FromAttributes;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

use crate::doc::extract_doc;

#[derive(FromAttributes)]
struct Opt {
    long: Option<syn::LitStr>,

    short: Option<syn::LitChar>,

    #[allow(dead_code)]
    short_only: Option<()>,
}

pub fn derive_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let attr = Opt::try_from_attributes(&input.attrs)?;

    let struct_name = &input.ident;
    let long = attr
        .as_ref()
        .and_then(|attr| attr.long.clone())
        .map_or_else(
            || struct_name.to_string().to_case(Case::Kebab),
            |lit_str| lit_str.value(),
        );

    let flag = match &attr.and_then(|opt| opt.short) {
        Some(short) => {
            quote! { cli_compose::schema::Flag::BothLongAndShort(#long.to_owned(), #short) }
        }
        None => quote! { cli_compose::schema::Flag::LongOnly(#long.to_owned()) },
    };

    let doc = extract_doc(&input.attrs);

    let sharp = syn::Token![#](proc_macro2::Span::call_site());

    Ok(quote! {
        impl cli_compose::schema::AsMember for #struct_name {
            fn handle(mut builder: cli_compose::schema::CliBuilder) -> cli_compose::schema::CliBuilder {
                use cli_compose::schema::{forwarded::quote::quote, AsOpt};

                let flag = format!("{}", <#struct_name as AsOpt>::flag());

                builder.ops.extend(quote! {
                    println!("ArgOpt {}", #sharp flag);
                });

                builder
            }
        }

        impl cli_compose::schema::AsOpt for #struct_name {
            fn flag() -> cli_compose::schema::Flag {
                #flag
            }

            fn description() -> String {
                #doc.to_owned()
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use quote::quote;

    fn test_opt_deriver(input: proc_macro2::TokenStream) -> anyhow::Result<String> {
        let tokens = super::derive_opt(input)?;

        crate::pretty_print::pretty_print_rust_code(tokens)
    }

    #[test]
    fn empty() {
        insta::assert_debug_snapshot!(test_opt_deriver(quote! {}));
    }
}
