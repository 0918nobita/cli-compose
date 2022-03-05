mod attr;
mod result;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Data;

use self::{
    attr::{extract_arg_opt_attr, ArgOptAttr},
    result::{ArgOptErr, ArgOptErrKind},
};
use crate::doc::extract_doc;

fn validate_struct(data_struct: &syn::DataStruct) -> Option<&syn::Field> {
    let unnamed = match data_struct {
        syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        } => Some(unnamed),
        _ => None,
    }?;

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Some(field),
        _ => None,
    }
}

pub fn derive_arg_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    match &derive_input.data {
        Data::Struct(struct_data) => {
            let field = validate_struct(struct_data).ok_or_else(|| {
                ArgOptErr::new(
                    ArgOptErrKind::InvalidTypeDef,
                    derive_input.to_token_stream(),
                )
            })?;

            let ArgOptAttr { long, short } = extract_arg_opt_attr(derive_input.attrs.iter())?;

            let struct_name = &derive_input.ident;
            let long = long.unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

            let flag = match short {
                Some(short) => quote! { cli_rs::Flag::BothLongAndShort(#long.to_owned(), #short) },
                None => quote! { cli_rs::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            let ty = field.ty.clone();

            Ok(quote! {
                impl cli_rs::AsArgOpt for #struct_name {
                    fn flag() -> cli_rs::Flag {
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

        Data::Enum(_) => {
            let ArgOptAttr { long, short } = extract_arg_opt_attr(derive_input.attrs.iter())?;

            let enum_name = &derive_input.ident;
            let long = long.unwrap_or_else(|| enum_name.to_string().to_case(Case::Kebab));

            let flag = match short {
                Some(short) => quote! { cli_rs::Flag::BothLongAndShort(#long.to_owned(), #short) },
                None => quote! { cli_rs::Flag::LongOnly(#long.to_owned()) },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            Ok(quote! {
                impl cli_rs::AsArgOpt for #enum_name {
                    fn flag() -> cli_rs::Flag {
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

        _ => Err(ArgOptErr::new(
            ArgOptErrKind::InvalidTypeDef,
            derive_input.to_token_stream(),
        )
        .into()),
    }
}
