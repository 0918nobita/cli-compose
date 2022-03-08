mod result;

use convert_case::{Case, Casing};
use darling::{
    ast::{self, Data},
    FromDeriveInput,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::result::{ArgOptErr, ArgOptErrKind};
use crate::doc::extract_doc;

#[derive(FromDeriveInput)]
#[darling(attributes(arg_opt), forward_attrs(doc))]
struct ArgOptInput {
    ident: syn::Ident,

    data: Data<syn::Ident, syn::Field>,

    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    long: Option<String>,

    #[darling(default)]
    short: Option<char>,

    #[darling(default)]
    use_default: bool,
}

pub fn derive_arg_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let input = match ArgOptInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => return Ok(err.write_errors()),
    };

    match &input.data {
        Data::Enum(_) => {
            let enum_name = &input.ident;

            let long = input
                .long
                .unwrap_or_else(|| input.ident.to_string().to_case(Case::Kebab));

            let flag = match input.short {
                Some(short) => {
                    quote! { cli_compose::Flag::BothLongAndShort(#long.to_owned(), #short) }
                }
                None => quote! { cli_compose::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(&input.attrs);

            Ok(quote! {
                impl cli_compose::AsArgOpt for #enum_name {
                    fn flag() -> cli_compose::Flag {
                        #flag
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        <#enum_name as std::str::FromStr>::from_str(s).ok()
                    }
                }
            })
        }

        Data::Struct(
            fields @ ast::Fields {
                style: ast::Style::Tuple,
                fields: fields_vec,
                ..
            },
        ) => {
            let field = match &fields_vec[..] {
                [field] => field,
                _ => {
                    return Err(ArgOptErr::new(
                        ArgOptErrKind::InvalidTypeDef,
                        fields.to_token_stream(),
                    )
                    .into())
                }
            };

            let struct_name = &input.ident;
            let long = input
                .long
                .unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

            let flag = match input.short {
                Some(short) => {
                    quote! { cli_compose::Flag::BothLongAndShort(#long.to_owned(), #short) }
                }
                None => quote! { cli_compose::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(&input.attrs);

            let ty = field.ty.clone();

            Ok(quote! {
                impl cli_compose::AsArgOpt for #struct_name {
                    fn flag() -> cli_compose::Flag {
                        #flag
                    }

                    fn description() -> String {
                        #doc.to_owned()
                    }

                    fn parse(s: &str) -> Option<Self> {
                        let val = <#ty as std::str::FromStr>::from_str(s).ok()?;
                        Some(#struct_name(val))
                    }
                }
            })
        }

        Data::Struct(fields) => {
            Err(ArgOptErr::new(ArgOptErrKind::InvalidTypeDef, fields.to_token_stream()).into())
        }
    }
}
