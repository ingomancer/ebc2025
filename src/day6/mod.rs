use std::collections::HashMap;

pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    let mut mentors = 0;
    let mut part1 = 0;
    for letter in lines.next().unwrap().chars() {
        if letter == 'A' {
            mentors += 1;
        } else if letter == 'a' {
            part1 += mentors;
        }
    }

    lines.next();
    let mut mentors = HashMap::new();
    let mut part2 = 0;
    for letter in lines.next().unwrap().chars() {
        if letter.is_uppercase() {
            mentors
                .entry(letter)
                .and_modify(|value| *value += 1)
                .or_insert(1);
        } else {
            part2 += mentors.get(&letter.to_ascii_uppercase()).unwrap_or(&0);
        }
    }

    lines.next();

    let camp = lines
        .next()
        .unwrap()
        .repeat(2)
        .chars()
        .collect::<Vec<char>>();
    let part3: usize = camp
        .iter()
        .enumerate()
        .map(|(i, letter)| {
            let start = i.saturating_sub(1000);
            let end = if i + 1000 >= camp.len() {
                camp.len() - 1
            } else {
                i + 1000
            };
            if letter.is_lowercase() {
                if (1000..11000).contains(&i) {
                    count_mentors_in_range(start, end, letter, &camp) * 999
                } else {
                    count_mentors_in_range(start, end, letter, &camp)
                }
            } else {
                0
            }
        })
        .sum();
    (format!("{part1}"), format!("{part2}"), format!("{part3}"))
}
fn count_mentors_in_range(start: usize, end: usize, novice: &char, camp: &[char]) -> usize {
    let mut mentors = 0;
    for i in start..=end {
        if camp[i] == novice.to_ascii_uppercase() {
            mentors += 1;
        }
    }
    mentors
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
ABabACacBCbca
# PART 2
ABabACacBCbca
# PART 3
AABCBABCABCabcabcABCCBAACBCa
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("5".to_owned(), "11".to_owned(), "3442321".to_owned())
        );
    }
}
