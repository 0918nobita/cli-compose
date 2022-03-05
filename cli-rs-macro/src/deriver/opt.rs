mod attr;
mod result;

use convert_case::{Case, Casing};
use derive_more::Display;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Data;

use self::{
    attr::{extract_opt_attr, OptAttr},
    result::{OptErr, OptErrKind},
};
use crate::doc::extract_doc;

fn validate_struct(data: &Data) -> Option<()> {
    match data {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => Some(()),
        _ => None,
    }
}

pub fn derive_opt(input: TokenStream) -> syn::Result<TokenStream> {
    let derive_input = syn::parse2::<syn::DeriveInput>(input)?;

    validate_struct(&derive_input.data)
        .ok_or_else(|| OptErr::new(OptErrKind::InvalidStruct, derive_input.to_token_stream()))?;

    let OptAttr { long, short } = extract_opt_attr(derive_input.attrs.iter())?;

    let struct_name = &derive_input.ident;
    let long = long.unwrap_or_else(|| struct_name.to_string().to_case(Case::Kebab));

    let flag = match short {
        Some(short) => quote! { cli_rs::Flag::BothLongAndShort(#long.to_owned(), #short) },
        None => quote! { cli_rs::Flag::LongOnly(#long.to_owned()) },
    };

    let doc = extract_doc(derive_input.attrs.iter());

    Ok(quote! {
        impl cli_rs::AsOpt for #struct_name {
            fn flag() -> cli_rs::Flag {
                #flag
            }

            fn description() -> String {
                #doc.to_owned()
            }
        }
    })
}

/// [`FlagNormalizer`] を使ってのみ生成可能な、正規化済みのフラグ
///
/// 短縮フラグが定義されている場合はそれも含む
#[derive(Hash)]
pub struct NormalizedFlag {
    short: Option<char>,
    long: String,
}

impl NormalizedFlag {
    #[allow(dead_code)]
    pub const fn short(&self) -> &Option<char> {
        &self.short
    }

    #[allow(dead_code)]
    pub const fn long(&self) -> &String {
        &self.long
    }
}

#[derive(Default)]
pub struct FlagNormalizer {
    inner: std::collections::HashMap<char, String>,
}

#[derive(Debug, Display)]
pub enum FlagError {
    #[display(
        fmt = "Duplicate short flag: `{} -> {}` and `{} -> {}`",
        _0,
        _1,
        _0,
        _2
    )]
    DuplicateShortFlag(char, String, String),
}

impl FlagNormalizer {
    /// short フラグと long フラグの対応を追加する
    #[allow(dead_code)]
    pub fn try_add(&mut self, short: &char, long: &str) -> Result<(), FlagError> {
        if let Some(dup_long) = self.inner.get(short) {
            return Err(FlagError::DuplicateShortFlag(
                *short,
                long.to_owned(),
                dup_long.to_owned(),
            ));
        }
        if self.inner.insert(*short, long.to_owned()).is_some() {
            panic!("Illigal inner state");
        }
        Ok(())
    }

    /// short フラグをもとに正規化されたフラグを得る
    #[allow(dead_code)]
    pub fn try_get(&self, short: &char) -> Option<NormalizedFlag> {
        let long = self.inner.get(short)?;
        Some(NormalizedFlag {
            short: Some(*short),
            long: long.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FlagNormalizer;

    #[test]
    fn test_try_add() {
        let mut normalizer = FlagNormalizer::default();
        assert!(normalizer.try_add(&'o', "output").is_ok());
        assert!(normalizer.try_add(&'v', "version").is_ok());
        assert!(normalizer.try_add(&'o', "output").is_err());
    }

    #[test]
    fn test_try_get() {
        let mut normalizer = FlagNormalizer::default();
        assert!(normalizer.try_add(&'o', "output").is_ok());
        assert!(matches!(
            normalizer.try_get(&'o'),
            Some(flag)
                if flag.long() == "output" && matches!(flag.short(), Some(c) if *c == 'o')
        ));
    }
}
