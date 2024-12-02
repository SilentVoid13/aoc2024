use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<u32>>;

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let mut v = Vec::new();
        for n in l.split_whitespace() {
            v.push(n.parse().unwrap());
        }
        res.push(v);
    }
    res
}

fn is_valid(v1: u32, v2: u32, is_increasing: bool) -> bool {
    let diff = v1.abs_diff(v2);
    (1..=3).contains(&diff) && (v1 < v2) == is_increasing
}

#[aoc(day2, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut total = 0;
    'rep: for report in input {
        let increasing = report[0] < report[1];
        for chunk in report.windows(2) {
            let [v1, v2] = chunk else {
                unreachable!();
            };
            if !is_valid(*v1, *v2, increasing) {
                continue 'rep;
            }
        }
        total += 1;
    }
    total
}

#[aoc(day2, part2)]
pub fn part2(input: &Input) -> u32 {
    let mut total = 0;
    for report in input {
        // not really optimized but good enough
        'rep: for idx in 0..report.len() {
            let mut i = 0;
            let mut j = 1;
            if i == idx {
                i += 1;
                j += 1;
            }
            if j == idx {
                j += 1;
            }
            let increasing = report[i] < report[j];

            while j < report.len() {
                let v1 = report[i];
                let v2 = report[j];
                if !is_valid(v1, v2, increasing) {
                    continue 'rep;
                }

                i += 1;
                j += 1;
                if i == idx {
                    i += 1;
                }
                if j == idx {
                    j += 1;
                }
            }
            total += 1;
            break;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 4);
    }
}
