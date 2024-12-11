use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashMap, HashMapExt};

type Input = Vec<usize>;

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Input {
    let res = input.split(" ").map(|x| x.parse().unwrap()).collect();
    res
}

fn run(base: &[usize], steps: usize) -> usize {
    let mut set = HashMap::new();
    for val in base {
        *set.entry(*val).or_default() += 1;
    }
    for _ in 0..steps {
        let mut new = HashMap::new();
        for (val, count) in set.iter() {
            if *val == 0 {
                *new.entry(1).or_default() += *count;
                continue;
            }
            let ndigits = val.ilog10() + 1;
            if ndigits % 2 == 0 {
                let divisor = 10_usize.pow(ndigits / 2);
                let left = val / divisor;
                let right = val % divisor;
                *new.entry(left).or_default() += *count;
                *new.entry(right).or_default() += *count;
            } else {
                *new.entry(*val * 2024).or_default() += *count;
            }
        }
        set = new;
    }
    let mut count = 0;
    for val in set.values() {
        count += val;
    }
    count
}

#[aoc(day11, part1)]
pub fn part1(input: &Input) -> usize {
    run(input, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: &Input) -> usize {
    run(input, 75)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"125 17"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 65601038650482);
    }
}
