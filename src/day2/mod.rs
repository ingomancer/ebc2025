use ebc2025::{add_tuple, div_tupl, mul_tuple};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    let a = lines.next().unwrap().split_once(",").unwrap();
    let a: (i32, i32) = (a.0.parse().unwrap(), a.1.parse().unwrap());
    let mut res = (0, 0);

    for _ in 0..3 {
        res = mul_tuple(res, res);
        res = div_tupl(res, (10, 10));
        res = add_tuple(res, a);
    }

    let part1 = format!("[{},{}]", res.0, res.1);

    let a = lines.nth(1).unwrap().split_once(",").unwrap();
    let a: (i64, i64) = (a.0.parse().unwrap(), a.1.parse().unwrap());
    let opposite = add_tuple(a, (1000, 1000));

    let res: u32 = (a.0..opposite.0 + 1)
        .step_by(10)
        .map(|i| {
            (a.1..opposite.1 + 1)
                .step_by(10)
                .map(|j| {
                    let mut check = (0, 0);
                    let p = (i, j);
                    let mut good = 1;
                    for _ in 0..100 {
                        check = mul_tuple(check, check);
                        check = div_tupl(check, (100000, 100000));
                        check = add_tuple(check, p);
                        if check.0 > 1000000
                            || check.0 < -1000000
                            || check.1 > 1000000
                            || check.1 < -1000000
                        {
                            good = 0;
                            break;
                        }
                    }
                    good
                })
                .sum::<u32>()
        })
        .sum();
    let part2 = format!("{res}");

    let a = lines.nth(1).unwrap().split_once(",").unwrap();
    let a: (i64, i64) = (a.0.parse().unwrap(), a.1.parse().unwrap());
    let opposite = add_tuple(a, (1000, 1000));

    let res: u32 = (a.0..opposite.0 + 1)
        .into_par_iter()
        .map(|i| {
            (a.1..opposite.1 + 1)
                .into_par_iter()
                .map(|j| {
                    let mut check = (0, 0);
                    let p = (i, j);
                    let mut good = 1;
                    for _ in 0..100 {
                        check = mul_tuple(check, check);
                        check = div_tupl(check, (100000, 100000));
                        check = add_tuple(check, p);
                        if check.0 > 1000000
                            || check.0 < -1000000
                            || check.1 > 1000000
                            || check.1 < -1000000
                        {
                            good = 0;
                            break;
                        }
                    }
                    good
                })
                .sum::<u32>()
        })
        .sum();
    (part1, part2, format!("{res}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
25,9
# PART 2
35300,-64910
# PART 3
35300,-64910
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            (
                "[357,862]".to_owned(),
                "4076".to_owned(),
                "406954".to_owned()
            )
        );
    }
}
