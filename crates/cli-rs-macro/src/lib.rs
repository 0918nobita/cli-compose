use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Fields};

fn extract_doc(attrs: impl Iterator<Item = Attribute>) -> String {
    attrs
        .filter_map(|attr| {
            if attr.style == AttrStyle::Outer && attr.path.is_ident("doc") {
                let raw = format!(
                    "{}",
                    attr.tokens
                        .into_iter()
                        .skip(1)
                        .collect::<proc_macro2::TokenStream>()
                );
                Some(raw.trim_matches('"').to_owned())
            } else {
                None
            }
        })
        .map(|doc| doc.trim_start().to_owned())
        .collect::<Vec<_>>()
        .join("\n")
}

#[proc_macro_derive(Arg)]
pub fn derive_arg(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let doc = extract_doc(derive_input.attrs.into_iter());
    println!("Doc: {:?}", doc);

    let data_struct = match &derive_input.data {
        Data::Struct(data) => data,
        _ => panic!("#[derive(Arg)] can only be applied to structs"),
    };

    let _unnamed_fields = match &data_struct.fields {
        Fields::Unnamed(fields) => fields,
        _ => panic!("#[derive(Arg)] can only be applied to structs with unnamed fields"),
    };

    let struct_name = derive_input.ident;

    quote! {
        impl cli_rs::ToArg for #struct_name {}
    }
    .into()
}
