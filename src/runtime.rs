pub use cli_compose_macro::use_cli;

pub trait AsCli<R> {
    fn parse(args: impl Iterator<Item = String>) -> R;
}

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

#[cfg(test)]
mod tests {
    use str_macro::str;

    #[test]
    fn test_parse_into_tokens() {
        insta::assert_debug_snapshot!(super::parse_into_tokens(
            vec![str!("example"), str!("--input-format"), str!("json")].into_iter()
        )
        .collect::<Vec<_>>());
    }
}
