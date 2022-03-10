mod result;

use convert_case::{Case, Casing};
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;

use crate::doc::extract_doc;

struct PosArgVariant;

impl darling::FromVariant for PosArgVariant {
    fn from_variant(variant: &syn::Variant) -> Result<Self, darling::Error> {
        match &variant.fields {
            syn::Fields::Unnamed(_) | syn::Fields::Named(_) => {
                let span = variant.ident.span();
                Err(darling::Error::unsupported_shape("no field").with_span(&span))
            }
            syn::Fields::Unit => Ok(PosArgVariant),
        }
    }
}

struct PosArgField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl darling::FromField for PosArgField {
    fn from_field(field: &syn::Field) -> Result<Self, darling::Error> {
        Ok(PosArgField {
            ident: field.ident.clone(),
            ty: field.ty.clone(),
        })
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(pos_arg), forward_attrs(doc))]
struct PosArgInput {
    ident: syn::Ident,

    data: darling::ast::Data<PosArgVariant, PosArgField>,

    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    name: Option<String>,

    #[allow(dead_code)]
    #[darling(default)]
    use_default: bool,
}

pub fn derive_pos_arg(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let input = match PosArgInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => return Ok(err.write_errors()),
    };

    let doc = extract_doc(&input.attrs);

    let struct_name = &input.ident;
    let struct_name_kebab_case = input
        .name
        .unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

    let parse_method = match &input.data {
        darling::ast::Data::Enum(_) => {
            quote! {
                fn parse(s: &str) -> Option<Self> {
                    <#struct_name as std::str::FromStr>::from_str(s).ok()
                }
            }
        }

        darling::ast::Data::Struct(fields) => match fields.iter().collect::<Vec<_>>()[..] {
            [field] => {
                let ty = &field.ty;
                if let Some(ident) = &field.ident {
                    quote! {
                        fn parse(s: &str) -> Option<Self> {
                            <#ty as std::str::FromStr>::from_str(s).ok().map(|v| Self { #ident: v })
                        }
                    }
                } else {
                    quote! {
                        fn parse(s: &str) -> Option<Self> {
                            <#ty as std::str::FromStr>::from_str(s).ok().map(Self)
                        }
                    }
                }
            }
            [] => return Err(syn::Error::new_spanned(input.ident, "missing field")),
            _ => {
                return Err(syn::Error::new_spanned(
                    input.ident,
                    "multiple fields are not allowed",
                ))
            }
        },
    };

    Ok(quote::quote! {
        impl cli_compose::schema::AsPosArg for #struct_name {
            fn name() -> String {
                #struct_name_kebab_case.to_owned()
            }

            fn description() -> String {
                #doc.to_owned()
            }

            #parse_method
        }
    })
}
