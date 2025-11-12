#[derive(Debug)]
struct Fishbone {
    left: Option<usize>,
    num: usize,
    right: Option<usize>,
}

impl Fishbone {
    fn new(num: usize) -> Self {
        Fishbone {
            num,
            left: None,
            right: None,
        }
    }
}

fn parse_spine(nums: &str) -> Vec<Fishbone> {
    let mut spine = vec![];
    for num in nums.split(",").map(|num| num.parse::<usize>().unwrap()) {
        if spine.is_empty() {
            spine.push(Fishbone::new(num));
        } else {
            let mut inserted = false;
            for bone in &mut spine {
                if num < bone.num && bone.left.is_none() {
                    bone.left = Some(num);
                    inserted = true;
                    break;
                } else if num > bone.num && bone.right.is_none() {
                    bone.right = Some(num);
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                spine.push(Fishbone::new(num))
            }
        }
    }
    spine
}

pub fn run(input: String) -> (String, String, String) {
    let mut lines = input.lines();
    let (_, nums) = lines.next().unwrap().split_once(":").unwrap();

    let spine = parse_spine(nums);

    let part1 = spine
        .iter()
        .map(|bone| bone.num.to_string())
        .collect::<String>();

    lines.next();

    let mut max = 0;
    let mut min = usize::MAX;

    while let Some((_, nums)) = lines.next().unwrap().split_once(":") {
        let spine = parse_spine(nums);
        let quality = spine
            .iter()
            .map(|bone| bone.num.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        if quality > max {
            max = quality;
        }

        if quality < min {
            min = quality;
        }
    }

    let mut swords = vec![];

    while let Some((id, nums)) = lines.next().unwrap().split_once(":") {
        let id = id.parse::<usize>().unwrap();
        let spine = parse_spine(nums);
        let quality = spine
            .iter()
            .map(|bone| bone.num.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let levels = spine
            .iter()
            .map(|bone| {
                let left = if bone.left.is_some() {
                    bone.left.unwrap().to_string()
                } else {
                    "".to_string()
                };
                let right = if bone.right.is_some() {
                    bone.right.unwrap().to_string()
                } else {
                    "".to_string()
                };
                (left + &bone.num.to_string() + &right).parse().unwrap()
            })
            .collect::<Vec<usize>>();

        swords.push((id, quality, levels))
    }

    swords.sort_by(|a, b| match a.1.cmp(&b.1) {
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => {
            for (a, b) in a.2.iter().zip(&b.2) {
                if *a < *b {
                    return std::cmp::Ordering::Less;
                } else if *a > *b {
                    return std::cmp::Ordering::Greater;
                }
            }
            a.0.cmp(&b.0)
        }
    });
    let checksum: usize = swords
        .iter()
        .rev()
        .enumerate()
        .map(|(i, sword)| (i + 1) * sword.0)
        .sum();
    (part1, format!("{}", max - min), format!("{checksum}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
58:5,3,7,8,9,10,4,5,7,8,8
# PART 2
1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5
# PART 3
1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("581078".to_owned(), "77053".to_owned(), "260".to_owned())
        );
    }
}
