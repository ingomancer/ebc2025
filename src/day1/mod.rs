use std::ops::Rem;

use ebc2025::Clamped;

pub fn run(input: String) -> (String, String, String) {
    let mut input = input.lines();
    let names: Vec<&str> = input.next().unwrap().split(",").collect();
    let instructions: Vec<&str> = input.nth(1).unwrap().split(",").collect();

    let mut pos = Clamped::new(0, 0, (names.len() - 1) as i32);

    for instruction in instructions {
        let (dir, len) = instruction.split_at(1);
        let len: i32 = len.parse().unwrap();
        if dir == "R" {
            pos = pos + len;
        } else {
            pos = pos - len;
        }
    }

    let part1 = names[pos.get() as usize].to_string();

    let names: Vec<&str> = input.nth(1).unwrap().split(",").collect();
    let instructions: Vec<&str> = input.nth(1).unwrap().split(",").collect();

    let mut pos = 0;

    for instruction in instructions {
        let (dir, len) = instruction.split_at(1);
        let len: usize = len.parse().unwrap();
        if dir == "R" {
            pos = (pos + len).rem(names.len());
        } else if len > pos {
            pos = names.len() - (len - pos);
        } else {
            pos -= len;
        }
    }
    let part2 = names[pos].to_string();

    let mut names: Vec<&str> = input.nth(1).unwrap().split(",").collect();
    let instructions: Vec<&str> = input.nth(1).unwrap().split(",").collect();

    for instruction in instructions {
        let (dir, len) = instruction.split_at(1);
        let len: usize = len.parse().unwrap();

        let pos = if dir == "R" {
            len % names.len()
        } else {
            (names.len() - len % names.len()) % names.len()
        };

        names.swap(0, pos);
    }

    let part3 = names[0].to_string();
    (part1, part2, part3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1
# PART 2
Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1
# PART 3
Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            (
                "Fyrryn".to_owned(),
                "Elarzris".to_owned(),
                "Drakzyph".to_owned()
            )
        );
    }
}
