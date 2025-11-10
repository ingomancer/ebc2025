use std::collections::{BTreeMap, BTreeSet};

pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    let mut crates = BTreeSet::new();
    for size in lines.next().unwrap().split(",") {
        crates.insert(size.parse::<u32>().unwrap());
    }
    let part1: u32 = crates.iter().sum();

    let mut crates = BTreeSet::new();
    for size in lines.nth(1).unwrap().split(",") {
        crates.insert(size.parse::<u32>().unwrap());
    }

    let part2: u32 = crates
        .iter()
        .enumerate()
        .map_while(|(i, num)| if i < 20 { Some(num) } else { None })
        .sum();

    let mut crates = BTreeMap::new();
    let mut part3 = 0;
    for size in lines.nth(1).unwrap().split(",") {
        let size = size.parse::<u32>().unwrap();
        let val = match crates.entry(size) {
            std::collections::btree_map::Entry::Vacant(vacant_entry) => *vacant_entry.insert(1),
            std::collections::btree_map::Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1) + 1
            }
        };
        if val > part3 {
            part3 = val;
        }
    }

    (format!("{part1}"), format!("{part2}"), format!("{part3}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
10,5,1,10,3,8,5,2,2
# PART 2
4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77
# PART 3
4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("29".to_owned(), "781".to_owned(), "3".to_owned())
        );
    }
}
