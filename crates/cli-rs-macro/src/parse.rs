use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Pat, Token, TypePath,
};

#[derive(Clone)]
struct ArgBind {
    pat: Pat,
    path: TypePath,
}

impl Parse for ArgBind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat = input.parse::<Pat>()?;

        input.parse::<Token![=]>()?;

        let path = input.parse::<TypePath>()?;

        Ok(Self { pat, path })
    }
}

enum ArgKind {
    Arg,
    Flag,
    FlagArg,
    Group,
}

impl TryFrom<syn::Ident> for ArgKind {
    type Error = syn::Error;

    fn try_from(ident: syn::Ident) -> Result<Self, Self::Error> {
        match &*ident.to_string() {
            "arg" => Ok(Self::Arg),

            "flag" => Ok(Self::Flag),

            "flag_arg" => Ok(Self::FlagArg),

            "group" => Ok(Self::Group),

            _ => Err(syn::Error::new_spanned(
                ident,
                "expected `arg`, `flag`, `flag_arg`, or `group`",
            )),
        }
    }
}

impl ToString for ArgKind {
    fn to_string(&self) -> String {
        match self {
            Self::Arg => "Arguments",

            Self::Flag => "Flags",

            Self::FlagArg => "Flag arguments",

            Self::Group => "Groups",
        }
        .to_owned()
    }
}

struct ArgBindGroup {
    kind: ArgKind,
    binds: Vec<ArgBind>,
}

impl Parse for ArgBindGroup {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;

        let kind = ArgKind::try_from(ident)?;

        let content;
        syn::braced!(content in input);

        let binds = content
            .parse_terminated::<ArgBind, Token![,]>(ArgBind::parse)?
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        Ok(Self { kind, binds })
    }
}

struct ArgTypes {
    args: Expr,
    arg_bind_groups: Vec<ArgBindGroup>,
}

impl Parse for ArgTypes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = input.call(Expr::parse_without_eager_brace)?;

        input.parse::<Token![,]>()?;

        let mut arg_bind_groups = Vec::<ArgBindGroup>::new();
        while !input.is_empty() {
            let arg_bind_group = input.parse::<ArgBindGroup>()?;
            arg_bind_groups.push(arg_bind_group);
        }

        Ok(Self {
            args,
            arg_bind_groups,
        })
    }
}

pub fn parse(input: TokenStream) -> syn::Result<TokenStream> {
    let ArgTypes {
        args,
        arg_bind_groups,
    } = syn::parse2::<ArgTypes>(input)?;

    let mut dump_code = TokenStream::new();

    for ArgBindGroup { kind, binds } in arg_bind_groups {
        let kind_str = kind.to_string();
        dump_code.extend(quote! { println!("[{}]", #kind_str); });

        for ArgBind { pat: _pat, path } in binds {
            let path_str = path.to_token_stream().to_string();

            dump_code.extend(match kind {
                ArgKind::Arg => quote! {
                    println!("    {}: {}", <#path as cli_rs::AsArg>::name(), <#path as cli_rs::AsArg>::description());
                },

                ArgKind::Flag => quote! {
                    let names = if let Some(short) = <#path as cli_rs::AsFlag>::short() {
                        format!("{}, {}", short, <#path as cli_rs::AsFlag>::long())
                    } else {
                        format!("{}", <#path as cli_rs::AsFlag>::long())
                    };
                    println!("    {}: {}", names, <#path as cli_rs::AsFlag>::description());
                },

                ArgKind::FlagArg => quote! {
                    let names = if let Some(short) = <#path as cli_rs::AsFlagArg>::short() {
                        format!("{}, {}", short, <#path as cli_rs::AsFlagArg>::long())
                    } else {
                        format!("{}", <#path as cli_rs::AsFlagArg>::long())
                    };
                    println!("    {}: {}", names, <#path as cli_rs::AsFlagArg>::description());
                },

                ArgKind::Group => quote! {
                    println!("    {}", #path_str);
                },
            });

            dump_code.extend(quote! { println!(); });
        }
    }

    Ok(quote! {
        {
            #dump_code
            let tokens = cli_rs::parse_into_tokens(#args).collect::<Vec<_>>();
            println!("tokens: {:?}", tokens);
        }
    })
}
