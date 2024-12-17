use gxhash::{HashMap, HashMapExt};

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>);

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Input {
    let (orders, lists) = input.split_once("\n\n").unwrap();
    let mut h: HashMap<usize, Vec<usize>> = HashMap::new();
    for l in orders.lines() {
        let (key, value) = l.split_once("|").unwrap();
        let key = key.parse().unwrap();
        let value = value.parse().unwrap();
        h.entry(key).or_default().push(value);
    }
    let lists = lists
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    (h, lists)
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    let (orders, lists) = input;
    let mut total = 0;
    'list: for list in lists {
        for (i, val) in list.iter().enumerate() {
            if let Some(poss) = orders.get(val) {
                // check all values before to see if invalid
                for val2 in list.iter().take(i) {
                    if poss.contains(val2) {
                        continue 'list;
                    }
                }
            }
        }
        total += list[list.len() / 2];
    }
    total
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> usize {
    let (orders, lists) = input;
    let lists = lists.clone();
    let mut total = 0;
    for mut list in lists {
        let mut invalid = false;
        // unoptimized bubble sort
        let mut swap = true;
        while swap {
            swap = false;
            for i in 0..list.len() {
                if let Some(poss) = orders.get(&list[i]) {
                    for j in 0..i {
                        if poss.contains(&list[j]) {
                            invalid = true;
                            swap = true;
                            list.swap(j, i);
                        }
                    }
                }
            }
        }
        if invalid {
            total += list[list.len() / 2];
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 123);
    }
}
