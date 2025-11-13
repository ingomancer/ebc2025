#[cfg(test)]
const TESTING: bool = true;

#[cfg(not(test))]
const TESTING: bool = false;

pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    let mut prev_nail: Option<i32> = None;
    let mut part1 = 0;
    let nails = if TESTING { 8 } else { 32 };
    for pos in lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
    {
        if let Some(prev_nail) = prev_nail
            && (prev_nail - pos).abs() == nails / 2
        {
            part1 += 1;
        }
        prev_nail = Some(pos);
    }

    lines.next();

    let mut prev_nail: Option<i32> = None;
    let mut part2 = 0;
    let mut strings = vec![];
    for pos in lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
    {
        if let Some(prev_nail) = prev_nail {
            for pair in &strings {
                if different_sides(side_of_line(prev_nail, *pair), side_of_line(pos, *pair))
                    == Crosses::Yes
                {
                    part2 += 1;
                }
            }
            let pair = if prev_nail < pos {
                (prev_nail, pos)
            } else {
                (pos, prev_nail)
            };
            strings.push(pair);
        }
        prev_nail = Some(pos);
    }

    lines.next();

    let mut prev_nail: Option<i32> = None;
    let mut strings = vec![];
    let nails = if TESTING { 8 } else { 256 };
    let mut part3 = 0;
    for pos in lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
    {
        if let Some(prev_nail) = prev_nail {
            let pair = if prev_nail < pos {
                (prev_nail, pos)
            } else {
                (pos, prev_nail)
            };
            strings.push(pair);
        }
        prev_nail = Some(pos);
    }

    for i in 1..=nails {
        for j in i..=nails {
            let mut count = 0;
            for pair in &strings {
                if different_sides(side_of_line(i, *pair), side_of_line(j, *pair)) == Crosses::Yes {
                    count += 1;
                }
                if &(i, j) == pair || &(j, i) == pair {
                    count += 1;
                }
            }
            if count > part3 {
                part3 = count;
            }
        }
    }

    (format!("{part1}"), format!("{part2}"), format!("{part3}"))
}

#[derive(PartialEq)]
enum Side {
    Left,
    Right,
    Neither,
}

#[derive(PartialEq)]
enum Crosses {
    Yes,
    No,
    Intersects,
}

fn side_of_line(dot: i32, line: (i32, i32)) -> Side {
    if dot == line.0 || dot == line.1 {
        Side::Neither
    } else if dot > line.0 && dot < line.1 {
        Side::Right
    } else {
        Side::Left
    }
}

fn different_sides(a: Side, b: Side) -> Crosses {
    if a == Side::Neither || b == Side::Neither {
        Crosses::Intersects
    } else if a != b {
        Crosses::Yes
    } else {
        Crosses::No
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
1,5,2,6,8,4,1,7,3
# PART 2
1,5,2,6,8,4,1,7,3,5,7,8,2
# PART 3
1,5,2,6,8,4,1,7,3,6
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("4".to_owned(), "21".to_owned(), "7".to_owned())
        );
    }
}
