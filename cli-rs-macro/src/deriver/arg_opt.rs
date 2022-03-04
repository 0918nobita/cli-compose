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
            let short = match short {
                Some(lit) => quote! { Some(cli_rs::ShortFlag::new(#lit)) },
                None => quote! { None },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            let ty = field.ty.clone();
            let struct_name = &derive_input.ident;
            let struct_name_kebab_case =
                long.unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

            Ok(quote! {
                impl cli_rs::AsArgOpt for #struct_name {
                    fn long() -> cli_rs::LongFlag {
                        cli_rs::LongFlag::new(#struct_name_kebab_case)
                    }

                    fn short() -> Option<cli_rs::ShortFlag> {
                        #short
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
            let short = match short {
                Some(lit) => quote! { Some(cli_rs::ShortFlag::new(#lit)) },
                None => quote! { None },
            };

            let doc = extract_doc(derive_input.attrs.iter());

            let enum_name = &derive_input.ident;
            let enum_name_kebab_case =
                long.unwrap_or_else(|| enum_name.to_string().to_case(Case::Kebab));

            Ok(quote! {
                impl cli_rs::AsArgOpt for #enum_name {
                    fn long() -> cli_rs::LongFlag {
                        cli_rs::LongFlag::new(#enum_name_kebab_case)
                    }

                    fn short() -> Option<cli_rs::ShortFlag> {
                        #short
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
