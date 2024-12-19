use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashMap, HashMapExt};

type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
pub fn parse(input: &str) -> Input {
    let mut designs = Vec::new();
    let mut rules = Vec::new();
    let (d, r) = input.split_once("\n\n").unwrap();
    for dval in d.split(", ") {
        designs.push(dval.to_string());
    }
    for rval in r.lines() {
        rules.push(rval.to_string());
    }
    (designs, rules)
}

fn dp(
    i: usize,
    designs: &Vec<String>,
    val: &str,
    cache: &mut HashMap<usize, usize>,
    p1: bool,
) -> usize {
    if let Some(&res) = cache.get(&i) {
        return res;
    }
    if i == val.len() {
        cache.insert(i, 1);
        return 1;
    }
    if i > val.len() {
        cache.insert(i, 0);
        return 0;
    }
    let mut res = 0;
    for d in designs {
        if val[i..].starts_with(d) {
            res += dp(i + d.len(), designs, val, cache, p1);
            // for p1 we don't need to check all designs
            if res > 0 && p1 {
                return res;
            }
        }
    }
    cache.insert(i, res);
    res
}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> usize {
    let (designs, rules) = input;
    let mut designs = designs.clone();
    designs.sort();
    let mut sum = 0;
    for rule in rules {
        let mut cache = HashMap::new();
        if dp(0, &designs, rule, &mut cache, true) > 0 {
            sum += 1;
        }
    }
    sum
}

#[aoc(day19, part2)]
pub fn part2(input: &Input) -> usize {
    let (designs, rules) = input;
    let mut designs = designs.clone();
    designs.sort();
    let mut sum = 0;
    for rule in rules {
        let mut cache = HashMap::new();
        sum += dp(0, &designs, rule, &mut cache, false);
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 16);
    }
}
