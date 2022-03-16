use std::fs;

use convert_case::{Case, Casing};
use derive_more::Display;
use proc_macro2::TokenStream;
use thiserror::Error;

pub use cli_compose_macro::{ArgOpt, Cli, FromKebabStr, MultiSelect, Opt, PosArg, SingleSelect};
pub use quote::{quote, ToTokens};
pub use syn::{parse_str, Ident, Type};

#[derive(Display)]
pub enum Flag {
    #[display(fmt = "--{}", _0)]
    LongOnly(String),

    #[display(fmt = "-{}", _0)]
    ShortOnly(char),

    #[display(fmt = "--{}, -{}", _0, _1)]
    BothLongAndShort(String, char),
}

pub trait AsMember {
    fn handle(builder: CliBuilder) -> CliBuilder;
}

pub trait AsPosArg: Sized + AsMember {
    fn name() -> String;

    fn description() -> String;

    fn parse(s: &str) -> Option<Self>;

    fn result() -> syn::Type;
}

pub trait AsArgOpt: Sized + AsMember {
    fn flag() -> Flag;

    fn description() -> String;

    fn parse(s: &str) -> Option<Self>;
}

pub trait AsOpt: AsMember {
    fn flag() -> Flag;

    fn description() -> String;
}

pub trait AsSingleSelect: AsMember {
    fn name() -> String;

    fn description() -> String;
}

pub trait AsMultiSelect: AsMember {
    fn name() -> String;

    fn description() -> String;
}

pub trait AsCliMeta {
    fn ident() -> syn::Ident;
}

pub fn ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

#[derive(Debug, Error)]
pub enum CliBuilderError {
    #[error("The base path is invalid")]
    InvalidBasePath,

    #[error("The result type is invalid")]
    InvalidResultTypeName,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct CliBuilder {
    base_path: syn::Path,
    cli_ty: syn::Ident,
    pub ops: TokenStream,
}

pub type CliBuilderResult<T> = Result<T, CliBuilderError>;

impl CliBuilder {
    pub fn new<Cli: AsCliMeta>(base_path: &str) -> CliBuilderResult<Self> {
        let base_path = syn::parse_str(base_path).map_err(|_| CliBuilderError::InvalidBasePath)?;

        Ok(CliBuilder {
            base_path,
            cli_ty: Cli::ident(),
            ops: TokenStream::new(),
        })
    }

    pub fn member<M: AsMember>(self) -> Self {
        M::handle(self)
    }

    pub fn build(self, result_type_name: &str) -> Result<(), CliBuilderError> {
        let out_dir = std::env::var("OUT_DIR").map_err(|e| CliBuilderError::Other(e.into()))?;

        let dest_dir = std::path::Path::new(&out_dir).join("cli_compose");

        fs::create_dir_all(&dest_dir).map_err(|e| CliBuilderError::Other(e.into()))?;

        let mut dest = dest_dir
            .as_path()
            .join(self.cli_ty.to_string().to_case(Case::Snake));
        dest.set_extension("rs");

        let result_type: syn::Ident =
            syn::parse_str(result_type_name).map_err(|_| CliBuilderError::InvalidResultTypeName)?;

        let base_path = self.base_path;
        let cli_ty = self.cli_ty;
        let ops = self.ops;

        let contents = quote! {
            struct #result_type {
            }

            impl cli_compose::runtime::AsCli<#result_type> for #base_path::#cli_ty {
                fn parse(args: impl Iterator<Item = String>) -> #result_type {
                    let tokens = cli_compose::runtime::parse_into_tokens(args).collect::<Vec<_>>();
                    println!("tokens: {:?}", tokens);
                    #ops
                    todo!()
                }
            }
        };

        fs::write(&dest, contents.to_string()).map_err(|e| CliBuilderError::Other(e.into()))
    }
}
