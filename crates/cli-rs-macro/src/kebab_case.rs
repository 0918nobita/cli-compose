pub fn upper_camel_to_kebab(str: &str) -> String {
    fn folder((first_skipped, mut cs): (bool, Vec<char>), c: char) -> (bool, Vec<char>) {
        if ('A'..='Z').contains(&c) {
            if !first_skipped {
                cs.push(c.to_ascii_lowercase());
                return (true, cs);
            }
            cs.push('-');
            cs.push(c.to_ascii_lowercase());
            return (true, cs);
        }

        cs.push(c);
        (first_skipped, cs)
    }

    str.chars()
        .fold((false, vec![]), folder)
        .1
        .into_iter()
        .collect()
}
