use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data};
use thiserror::Error;

#[derive(Debug, Error)]
enum DeriverError {
    #[error("#[derive(Arg)] can only be applied to structs with single unnamed field")]
    InvalidStruct,
}

fn try_get_single_line_doc(attr: Attribute) -> Option<String> {
    let tokens = match attr {
        Attribute {
            path,
            style: syn::AttrStyle::Outer,
            tokens,
            ..
        } if path.is_ident("doc") => tokens,
        _ => return None,
    };
    let doc = tokens
        .into_iter()
        .skip(1)
        .collect::<proc_macro2::TokenStream>();
    Some(format!("{}", doc).trim_matches('"').trim_start().to_owned())
}

fn extract_doc(attrs: impl Iterator<Item = Attribute>) -> String {
    attrs
        .filter_map(try_get_single_line_doc)
        .collect::<Vec<_>>()
        .join("\n")
}

fn validate_struct(data: &Data) -> Result<&syn::Field, DeriverError> {
    let unnamed = match data {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        }) => unnamed,

        _ => return Err(DeriverError::InvalidStruct),
    };

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Ok(field),

        _ => Err(DeriverError::InvalidStruct),
    }
}

#[proc_macro_derive(Arg)]
/// コマンドライン引数
pub fn derive_arg(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let doc = extract_doc(derive_input.attrs.into_iter());

    let field = validate_struct(&derive_input.data).unwrap_or_else(|err| panic!("{}", err));

    let ty = field.ty.clone();
    let struct_name = derive_input.ident;
    let struct_name_lowercase = struct_name.to_string().to_lowercase();

    quote! {
        impl cli_rs::ToArg for #struct_name {
            fn name() -> String {
                #struct_name_lowercase.to_owned()
            }

            fn description() -> String {
                #doc.to_owned()
            }

            fn parse(str: &str) -> Option<Self> {
                use std::str::FromStr;
                #ty::from_str(str).ok().map(|v| #struct_name(v))
            }
        }
    }
    .into()
}

#[proc_macro_derive(Flag)]
/// 値を持たないフラグ
pub fn derive_flag(_: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro_derive(FlagArg)]
/// 値を要求するフラグ
pub fn derive_flag_arg(_: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro_derive(ArgGroup)]
/// 引数グループ
pub fn derive_arg_group(_: TokenStream) -> TokenStream {
    quote! {}.into()
}
