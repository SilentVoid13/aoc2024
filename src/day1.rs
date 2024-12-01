use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<u32>, Vec<u32>);

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Input {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    for l in input.lines() {
        let mut it = l.split_whitespace().map(|v| v.parse::<u32>().unwrap());
        l1.push(it.next().unwrap());
        l2.push(it.next().unwrap());
    }
    l1.sort();
    l2.sort();
    (l1, l2)
}

#[aoc(day1, part1)]
pub fn part1((l1, l2): &Input) -> u32 {
    let mut total = 0;
    for (v1, v2) in l1.iter().zip(l2.iter()) {
        total += v2.abs_diff(*v1);
    }
    total
}

#[aoc(day1, part2)]
pub fn part2((l1, l2): &Input) -> u32 {
    let mut i1 = 0;
    let mut i2 = 0;

    let mut total = 0;
    // we use the fact that both list are sorted
    while i1 < l1.len() && i2 < l2.len() {
        let mut mult = 0;
        let v = l1[i1];

        while i2 < l2.len() && l2[i2] <= v {
            if l2[i2] == v {
                mult += 1;
            }
            i2 += 1;
        }
        while i1 < l1.len() && l1[i1] == v {
            i1 += 1;
            total += mult * v;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 31);
    }
}
