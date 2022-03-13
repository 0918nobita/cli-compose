use syn::{parse, Ident, Path, Token};

pub struct CliDef {
    pub cli_ty: Ident,
    pub res_ty: Ident,
    pub members: Vec<Path>,
}

impl parse::Parse for CliDef {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let cli_ty = input.parse()?;

        input.parse::<Token![->]>()?;

        let res_ty = input.parse()?;

        let members;
        syn::braced!(members in input);
        let members = members
            .parse_terminated::<Path, Token![,]>(Path::parse)?
            .into_iter()
            .collect::<Vec<_>>();

        Ok(Self {
            cli_ty,
            res_ty,
            members,
        })
    }
}
