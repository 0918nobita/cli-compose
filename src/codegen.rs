use std::fs;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use thiserror::Error;

use crate::schema::{AsArgOpt, AsCliMeta, AsMultiSelect, AsOpt, AsPosArg, AsSingleSelect};

pub trait AsMember<Tag> {
    fn handle(builder: CliBuilder) -> CliBuilder;
}

pub struct PosArgTag;

impl<T: AsPosArg> AsMember<PosArgTag> for T {
    fn handle(mut builder: CliBuilder) -> CliBuilder {
        let name = T::name();
        builder.ops.extend(quote! {
            println!("PosArg {}", #name);
        });
        builder
    }
}

pub struct ArgOptTag;

impl<T: AsArgOpt> AsMember<ArgOptTag> for T {
    fn handle(mut builder: CliBuilder) -> CliBuilder {
        let flag = format!("{}", T::flag());
        builder.ops.extend(quote! {
            println!("ArgOpt {}", #flag);
        });
        builder
    }
}

pub struct OptTag;

impl<T: AsOpt> AsMember<OptTag> for T {
    fn handle(mut builder: CliBuilder) -> CliBuilder {
        let flag = format!("{}", T::flag());
        builder.ops.extend(quote! {
            println!("   Opt {}", #flag);
        });
        builder
    }
}

pub struct SingleSelectTag;

impl<T: AsSingleSelect> AsMember<SingleSelectTag> for T {
    fn handle(_builder: CliBuilder) -> CliBuilder {
        todo!()
    }
}

pub struct MultiSelectTag;

impl<T: AsMultiSelect> AsMember<MultiSelectTag> for T {
    fn handle(_builder: CliBuilder) -> CliBuilder {
        todo!()
    }
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
    ops: TokenStream,
}

type CliBuilderResult<T> = Result<T, CliBuilderError>;

impl CliBuilder {
    pub fn new<Cli: AsCliMeta>(base_path: &str) -> CliBuilderResult<Self> {
        let base_path = syn::parse_str(base_path).map_err(|_| CliBuilderError::InvalidBasePath)?;
        Ok(CliBuilder {
            base_path,
            cli_ty: Cli::ident(),
            ops: TokenStream::new(),
        })
    }

    pub fn member<M: AsMember<T>, T>(self) -> Self {
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

pub fn define_cli<Cli: AsCliMeta>(base_path: &str) -> CliBuilderResult<CliBuilder> {
    CliBuilder::new::<Cli>(base_path)
}
