pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines().peekable();
    let first_gear = lines.next().unwrap().parse::<f64>().unwrap();
    let mut last_gear = 0.0;
    while !lines.peek().unwrap().starts_with("#") {
        last_gear = lines.next().unwrap().parse().unwrap();
    }

    let part1 = (first_gear / last_gear * 2025.0) as u64;
    lines.next();
    let first_gear = lines.next().unwrap().parse::<f64>().unwrap();
    let mut last_gear = 0.0;
    while !lines.peek().unwrap().starts_with("#") {
        last_gear = lines.next().unwrap().parse().unwrap();
    }

    let part2 = (10000000000000.0 / (first_gear / last_gear)).round();

    lines.next();
    let mut prev_gear = lines.next().unwrap().parse::<f64>().unwrap();
    let mut mult = 1.0;
    for line in lines {
        match line.split_once("|") {
            Some(gears) => {
                let gear1 = gears.0.parse::<f64>().unwrap();
                let gear2 = gears.1.parse::<f64>().unwrap();
                mult *= prev_gear / gear1;
                prev_gear = gear2;
            }
            None => {
                let gear = line.parse::<f64>().unwrap();
                mult *= prev_gear / gear;
            }
        }
    }
    (
        format!("{part1}"),
        format!("{part2}"),
        format!("{}", (mult * 100.0) as u64),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
102
75
50
35
13
# PART 2
102
75
50
35
13
# PART 3
5
7|21
18|36
27|27
10|50
10|50
11
"};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            (
                "15888".to_owned(),
                "1274509803922".to_owned(),
                "6818".to_owned()
            )
        );
    }
}
