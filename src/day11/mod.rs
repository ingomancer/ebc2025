use std::usize;

pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    let mut columns = vec![];
    while let Ok(ducks) = lines.next().unwrap().parse::<usize>() {
        columns.push(ducks);
    }

    let (part1, _) = balance_ducks(columns, Some(10));

    let mut columns = vec![];
    while let Ok(ducks) = lines.next().unwrap().parse::<usize>() {
        columns.push(ducks);
    }
    let (_, part2) = balance_ducks(columns, None);

    let mut columns = vec![];
    for next in lines {
        if let Ok(ducks) = next.parse() {
            columns.push(ducks);
        }
    }
    let ducks_per_column = columns.iter().sum::<usize>() / columns.len();
    let missing_ducks = columns
        .iter()
        .map(|x| {
            if *x < ducks_per_column {
                ducks_per_column - x
            } else {
                0
            }
        })
        .sum::<usize>();
    (
        format!("{part1}"),
        format!("{part2}"),
        format!("{missing_ducks}"),
    )
}

fn balance_ducks(mut columns: Vec<usize>, cycles: Option<usize>) -> (usize, usize) {
    let mut phase2 = false;
    let mut loops = 0;
    loop {
        let mut moved = false;
        for i in 0..columns.len() - 1 {
            if !phase2 {
                if columns[i] > columns[i + 1] {
                    moved = true;
                    columns[i] -= 1;
                    columns[i + 1] += 1;
                }
            } else if columns[i] < columns[i + 1] {
                moved = true;
                columns[i] += 1;
                columns[i + 1] -= 1;
            }
        }
        if !moved {
            if phase2 {
                break;
            } else {
                phase2 = true;
            }
        }

        if let Some(cycles) = cycles
            && cycles == loops
        {
            break;
        }
        loops += 1;
    }
    (
        columns
            .iter()
            .enumerate()
            .map(|(index, ducks)| (index + 1) * ducks)
            .sum(),
        loops - 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
9
1
1
4
9
6
# PART 2
805
706
179
48
158
150
232
885
598
524
423
# PART 3
9
1
1
4
9
6
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("109".to_owned(), "1579".to_owned(), "11".to_owned())
        );
    }
}
