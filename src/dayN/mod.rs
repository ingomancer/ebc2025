pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    (format!(""), format!(""), format!(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"

# PART 2

# PART 3

        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("".to_owned(), "".to_owned(), "".to_owned())
        );
    }
}
