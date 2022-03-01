pub fn upper_camel_to_kebab(str: &str) -> String {
    str.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if ('A'..='Z').contains(&c) {
                if i != 0 {
                    vec!['-', c.to_ascii_lowercase()]
                } else {
                    vec![c.to_ascii_lowercase()]
                }
            } else {
                vec![c]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_upper_camel_to_kebab() {
        assert_eq!(super::upper_camel_to_kebab("FooBar"), "foo-bar");
        assert_eq!(super::upper_camel_to_kebab("fooBarBaz"), "foo-bar-baz");
    }
}
