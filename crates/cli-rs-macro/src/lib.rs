use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Fields};

fn try_get_single_line_doc(attr: Attribute) -> Option<String> {
    let tokens = match attr {
        Attribute {
            path,
            style: AttrStyle::Outer,
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

#[proc_macro_derive(Arg)]
pub fn derive_arg(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let doc = extract_doc(derive_input.attrs.into_iter());

    let data_struct = match &derive_input.data {
        Data::Struct(data) => data,
        _ => panic!("#[derive(Arg)] can only be applied to structs"),
    };

    let unnamed_fields = match &data_struct.fields {
        Fields::Unnamed(fields) => fields,
        _ => panic!("#[derive(Arg)] can only be applied to structs with unnamed fields"),
    };

    if unnamed_fields.unnamed.len() != 1 {
        panic!("#[derive(Arg)] can only be applied to structs with exactly one unnamed field");
    }

    let first_field = unnamed_fields
        .unnamed
        .first()
        .expect("Unnamed fields should not be empty");

    let ty = first_field.ty.clone();

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
