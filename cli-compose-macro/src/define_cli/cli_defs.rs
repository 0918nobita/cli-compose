use syn::parse;

use super::cli_def::CliDef;

pub struct CliDefs(Vec<CliDef>);

impl<'a> std::iter::IntoIterator for &'a CliDefs {
    type Item = &'a CliDef;
    type IntoIter = std::slice::Iter<'a, CliDef>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl parse::Parse for CliDefs {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let defs = input
            .parse_terminated::<CliDef, syn::Token![,]>(CliDef::parse)?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(Self(defs))
    }
}
