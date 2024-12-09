use aoc_runner_derive::{aoc, aoc_generator};

type Input = u32;

#[aoc_generator(dayx)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
    }
    0
}

#[aoc(dayx, part1)]
pub fn part1(input: &Input) -> u32 {
    0
}

#[aoc(dayx, part2)]
pub fn part2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
