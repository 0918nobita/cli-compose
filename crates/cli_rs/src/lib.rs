use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Arg)]
pub fn derive_arg(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);

    let data_struct = match &item.data {
        Data::Struct(data) => data,
        _ => panic!("#[derive(Arg)] can only be applied to structs"),
    };

    let _unnamed_fields = match &data_struct.fields {
        Fields::Unnamed(fields) => fields,
        _ => panic!("#[derive(Arg)] can only be applied to structs with unnamed fields"),
    };

    let struct_name = item.ident;

    quote! {
        impl #struct_name {
            fn answer() -> u32 { 42 }
        }
    }
    .into()
}
