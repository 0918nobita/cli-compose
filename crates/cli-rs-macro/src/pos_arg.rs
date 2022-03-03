use proc_macro2::TokenStream;
use syn::{Data, NestedMeta};
use thiserror::Error;

use crate::{attr_meta::extract_meta, doc::extract_doc, kebab_case::upper_camel_to_kebab};

#[derive(Debug, Error)]
enum PosArgError {
    #[error("#[derive(PosArg)] can only be applied to structs with single unnamed field")]
    InvalidStruct,
}

fn validate_struct(data: &Data) -> Result<&syn::Field, PosArgError> {
    let unnamed = match data {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        }) => unnamed,

        _ => return Err(PosArgError::InvalidStruct),
    };

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Ok(field),

        _ => Err(PosArgError::InvalidStruct),
    }
}

struct ArgAttr {
    name: Option<String>,
}

// HACK: 可読性を上げたい
fn extract_arg_attr<'a>(
    attrs: impl Iterator<Item = &'a syn::Attribute> + 'a,
) -> syn::Result<ArgAttr> {
    let mut name: Option<String> = None;

    for nested_meta in extract_meta(attrs, "arg") {
        let meta = match nested_meta {
            NestedMeta::Meta(meta) => meta,
            _ => {
                return Err(syn::Error::new_spanned(
                    nested_meta,
                    "Literals in #[pos_arg(..)] are not allowed",
                ))
            }
        };

        let syn::MetaNameValue { path, lit, .. } = match meta {
            syn::Meta::NameValue(name_value) => name_value,
            _ => {
                return Err(syn::Error::new_spanned(
                    meta,
                    "Metadata in #[pos_arg(..)] is invalid",
                ))
            }
        };

        if path.is_ident("name") {
            let lit = match lit {
                syn::Lit::Str(lit) => lit,
                _ => {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "#[pos_arg(name = ..)] must be a string literal",
                    ))
                }
            };
            name = Some(lit.value());
        } else {
            return Err(syn::Error::new_spanned(
                path,
                "Unexpected key in #[pos_arg(..)]",
            ));
        }
    }

    Ok(ArgAttr { name })
}

pub fn derive_pos_arg(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    let field = validate_struct(&derive_input.data)
        .map_err(|err| syn::Error::new_spanned(&derive_input, err.to_string()))?;

    let ArgAttr { name } = extract_arg_attr(derive_input.attrs.iter())?;

    let doc = extract_doc(derive_input.attrs.iter());

    let _ty = field.ty.clone();
    let struct_name = derive_input.ident;
    let struct_name_kebab_case =
        name.unwrap_or_else(|| upper_camel_to_kebab(&struct_name.to_string()));

    Ok(quote::quote! {
        impl cli_rs::AsPosArg for #struct_name {
            fn name() -> String {
                #struct_name_kebab_case.to_owned()
            }

            fn description() -> String {
                #doc.to_owned()
            }
        }
    })
}
