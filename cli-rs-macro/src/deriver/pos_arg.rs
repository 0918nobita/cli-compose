mod attr;
mod result;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Data;

use self::{
    attr::{extract_pos_arg_attr, PosArgAttr},
    result::{PosArgErr, PosArgErrKind},
};
use crate::doc::extract_doc;

fn validate_struct(data: &Data) -> Option<&syn::Field> {
    let unnamed = match data {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
            ..
        }) => Some(unnamed),
        _ => None,
    }?;

    match unnamed.iter().collect::<Vec<_>>()[..] {
        [field] => Some(field),
        _ => None,
    }
}

pub fn derive_pos_arg(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    let field = validate_struct(&derive_input.data).ok_or_else(|| {
        PosArgErr::new(PosArgErrKind::InvalidStruct, derive_input.to_token_stream())
    })?;

    let PosArgAttr { name } = extract_pos_arg_attr(derive_input.attrs.iter())?;

    let doc = extract_doc(derive_input.attrs.iter());

    let _ty = field.ty.clone();
    let struct_name = derive_input.ident;
    let struct_name_kebab_case =
        name.unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

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
