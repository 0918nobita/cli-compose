pub use cli_compose_macro::use_cli;

#[derive(Debug, PartialEq)]
pub enum Token {
    Long(String),
    Short(char),
    Value(String),
}

pub fn parse_into_tokens<A>(args: A) -> impl Iterator<Item = Token>
where
    A: Iterator<Item = String>,
{
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
