use aoc_runner_derive::{aoc, aoc_generator};

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

#[inline]
fn p_idx(v: &[usize; 4]) -> usize {
    v[0] * 19usize.pow(3) + v[1] * 19usize.pow(2) + v[2] * 19 + v[3]
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
    let mut map = vec![0; 19usize.pow(4)];
    for &secret in input {
        let mut seen = vec![false; 19usize.pow(4)];
        let mut val = secret;
        let mut price_changes = [0usize; 4];
        let mut last = secret % 10;
        for i in 0..2000 {
            val = hash(val);
            let price = val % 10;
            // we avoid negatives
            let change = 9 + price - last;
            last = price;

            // shift the array
            price_changes = [price_changes[1], price_changes[2], price_changes[3], change];
            let p_idx = p_idx(&price_changes);

            if i >= 3 && !seen[p_idx] {
                seen[p_idx] = true;
                map[p_idx] += price;
            }
        }
    }
    let best = map.iter().max().unwrap();
    *best
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
