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
    use insta::assert_debug_snapshot;
    use str_macro::str;

    use super::parse_into_tokens;

    #[test]
    fn test_long_flag() {
        assert_debug_snapshot!(parse_into_tokens(
            vec![str!("example"), str!("--input-format"), str!("json")].into_iter()
        )
        .collect::<Vec<_>>());
    }

    #[test]
    fn test_short_flags() {
        assert_debug_snapshot!(parse_into_tokens(
            vec![str!("example"), str!("-vo"), str!("out.json")].into_iter()
        )
        .collect::<Vec<_>>())
    }
}
