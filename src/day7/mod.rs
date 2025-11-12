use std::collections::{HashMap, HashSet};

pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    let names = lines.next().unwrap().split(",").collect::<Vec<&str>>();
    lines.next();
    let mut rules = HashMap::new();
    while let Some((letter, allowed)) = lines.next().unwrap().split_once(">") {
        let mut allowed_letters = HashSet::new();
        for rule in allowed.chars().filter(|r| *r != ',') {
            allowed_letters.insert(rule);
        }
        rules.insert(letter.trim().chars().next().unwrap(), allowed_letters);
    }

    let mut part1 = "";

    for name in names {
        let mut prev_letter: Option<char> = None;
        let mut name_okay = true;
        for letter in name.chars() {
            if let Some(prev_letter) = prev_letter
                && !rules.get(&prev_letter).unwrap().contains(&letter)
            {
                name_okay = false;
            }
            prev_letter = Some(letter);
        }
        if name_okay {
            part1 = name;
        }
    }

    let names = lines.next().unwrap().split(",").collect::<Vec<&str>>();
    lines.next();
    let mut rules = HashMap::new();
    while let Some((letter, allowed)) = lines.next().unwrap().split_once(">") {
        let mut allowed_letters = HashSet::new();
        for rule in allowed.chars().filter(|r| *r != ',') {
            allowed_letters.insert(rule);
        }
        rules.insert(letter.trim().chars().next().unwrap(), allowed_letters);
    }

    let mut part2 = 0;

    for (index, name) in names.iter().enumerate() {
        let mut prev_letter: Option<char> = None;
        let mut name_okay = true;
        for letter in name.chars() {
            if let Some(prev_letter) = prev_letter
                && !rules.get(&prev_letter).unwrap().contains(&letter)
            {
                name_okay = false;
            }
            prev_letter = Some(letter);
        }
        if name_okay {
            part2 += index + 1;
        }
    }

    let mut names = vec![];
    'outer: for name in lines.next().unwrap().split(",") {
        for oldname in &names {
            if name.starts_with(oldname) {
                continue 'outer;
            }
        }
        names.push(name);
    }
    lines.next();
    let mut rules = HashMap::new();
    while let Some((letter, allowed)) = lines.next().unwrap().split_once(">") {
        let mut allowed_letters = HashSet::new();
        for rule in allowed.chars().filter(|r| *r != ',' && *r != ' ') {
            allowed_letters.insert(rule);
        }
        rules.insert(letter.trim().chars().next().unwrap(), allowed_letters);
    }

    let mut part3 = 0;

    for name in names {
        let mut prev_letter: Option<char> = None;
        let mut name_okay = true;
        for letter in name.chars() {
            if let Some(prev_letter) = prev_letter
                && !rules
                    .get(&prev_letter)
                    .unwrap_or(&HashSet::new())
                    .contains(&letter)
            {
                name_okay = false;
            }
            prev_letter = Some(letter);
        }
        if name_okay {
            part3 += count_possible_endings(prev_letter.unwrap(), name.len(), &rules);
        }
    }

    (part1.to_string(), format!("{part2}"), format!("{part3}"))
}

fn count_possible_endings(letter: char, len: usize, rules: &HashMap<char, HashSet<char>>) -> usize {
    if len == 11 {
        1
    } else {
        let mut sublengths = 0;
        for rule in rules.get(&letter).unwrap_or(&HashSet::new()) {
            sublengths += count_possible_endings(*rule, len + 1, rules);
        }
        if len < 7 { sublengths } else { sublengths + 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h
# PART 2
Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i
# PART 3
Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("Oroneth".to_owned(), "23".to_owned(), "1154".to_owned())
        );
    }
}
