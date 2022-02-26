pub use cli_rs_macro::{parse, Arg, Flag, FlagArg, Group};

#[derive(Clone, Debug)]
pub enum ArgMetadatum {
    Flag {
        long: String,
        short: Option<char>,
        description: String,
    },
    FlagArg {
        long: String,
        short: Option<char>,
        description: String,
    },
    Arg {
        name: String,
        description: String,
    },
    ArgGroup(Vec<ArgMetadatum>),
}

pub trait ToArgMetadatum {
    fn metadatum() -> ArgMetadatum;
}

pub trait AsFlagArg: Sized {
    fn parse(s: &str) -> Option<Self>;
}

#[derive(Debug)]
pub enum Token {
    Long(String),
    Short(char),
    Value(String),
}

pub fn parse_into_tokens(args: impl Iterator<Item = String>) -> impl Iterator<Item = Token> {
    args.skip(1).flat_map(|arg| {
        if let Some(flag) = arg.strip_prefix("--") {
            return vec![Token::Long(flag.to_owned())];
        }
        if let Some(cs) = arg.strip_prefix('-') {
            return cs.chars().map(Token::Short).collect::<Vec<_>>();
        }
        vec![Token::Value(arg)]
    })
}
