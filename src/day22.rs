use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashMap, HashMapExt, HashSet, HashSetExt};

type Input = Vec<usize>;

#[aoc_generator(day22)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        res.push(l.parse().unwrap());
    }
    res
}

#[inline]
fn hash(mut val: usize) -> usize {
    let r = val << 6;
    val ^= r;
    val &= 0xffffff;
    let r = val >> 5;
    val ^= r;
    val &= 0xffffff;
    let r = val << 11;
    val ^= r;
    val &= 0xffffff;
    val
}

#[aoc(day22, part1)]
pub fn part1(input: &Input) -> usize {
    let mut sum = 0;
    for &secret in input {
        let mut val = secret;
        for _ in 0..2000 {
            val = hash(val);
        }
        sum += val;
    }
    sum
}

#[aoc(day22, part2)]
pub fn part2(input: &Input) -> usize {
    let mut map: HashMap<[isize; 4], usize> = HashMap::new();
    for &secret in input {
        let mut seen = HashSet::new();
        let mut val = secret;
        let mut price_changes = [0isize; 4];
        let mut last = (secret % 10) as isize;
        for i in 0..2000 {
            val = hash(val);
            let price = val % 10;
            let change = price as isize - last;
            last = price as isize;
            price_changes = [price_changes[1], price_changes[2], price_changes[3], change];
            if i >= 3 && seen.insert(price_changes) {
                *map.entry(price_changes).or_default() += price;
            }
        }
    }
    let best = map.iter().max_by_key(|&(_, &v)| v).unwrap();
    *best.1
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"1
10
100
2024"#;

    const EXAMPLE2: &str = r#"1
2
3
2024"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2)), 23);
    }
}
