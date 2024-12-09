use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<usize>;

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Input {
    let values: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c as usize - '0' as usize)
        .collect();
    values
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> usize {
    let values = input.clone();
    let mut id = 0;
    let mut buckets = vec![vec![]; values.len()];
    let mut spaces = vec![];

    // first pass to create buckets
    for ci in 0..values.len() {
        if ci % 2 == 0 {
            buckets[ci].push((id, values[ci]));
            id += 1;
        } else if values[ci] != 0 {
            spaces.push((ci, values[ci]));
        }
    }

    // fill buckets
    let mut cur_space = 0;
    for ci in (0..values.len()).rev() {
        if ci % 2 == 1 {
            if !buckets[ci].is_empty() {
                break;
            }
            continue;
        }
        let (id, mut n) = buckets[ci][0];
        let base_n = n;

        while n != 0 && cur_space < spaces.len() {
            let (space_bucket, nspaces) = &mut spaces[cur_space];
            assert_ne!(*nspaces, 0);

            let new_bucket = &mut buckets[*space_bucket];
            new_bucket.push((id, n.min(*nspaces)));

            let new_n = n.saturating_sub(*nspaces);
            *nspaces = nspaces.saturating_sub(n);
            n = new_n;

            if *nspaces == 0 {
                cur_space += 1;
            }
        }
        buckets[ci][0].1 = n;
        spaces.push((ci, base_n - n));
    }

    let mut sum = 0;
    let mut i = 0;
    for bucket in buckets {
        for (id, n) in bucket {
            if n == 0 {
                continue;
            }
            let s1 = id * i * (n) + id * ((n) * (n - 1) / 2);
            sum += s1;
            i += n;
        }
    }
    sum
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> usize {
    let values = input.clone();
    let mut id = 0;
    let mut buckets = vec![vec![]; values.len()];
    let mut spaces = vec![(0, 0); values.len()];

    // first pass to create buckets
    for ci in 0..values.len() {
        if ci % 2 == 0 {
            buckets[ci].push((id, values[ci]));
            id += 1;
        } else if values[ci] != 0 {
            spaces[ci] = (ci, values[ci]);
        }
    }

    // fill buckets
    for ci in (0..values.len()).rev() {
        if ci % 2 == 1 {
            continue;
        }
        let (id, mut n) = buckets[ci][0];
        let base_n = n;

        let mut cur_space = 1;

        while cur_space < spaces.len() && cur_space < ci {
            let (space_bucket, nspaces) = &mut spaces[cur_space];
            if *nspaces == 0 || n > *nspaces {
                cur_space += 2;
                continue;
            }
            if *space_bucket > ci {
                break;
            }

            let new_bucket = &mut buckets[*space_bucket];
            new_bucket.push((id, n.min(*nspaces)));

            let new_n = n.saturating_sub(*nspaces);
            *nspaces = nspaces.saturating_sub(n);
            n = new_n;

            buckets[ci][0].1 = n;
            spaces[ci] = (ci, base_n - n);

            break;
        }
    }

    let mut sum = 0;
    let mut i = 0;
    for bi in 0..values.len() {
        for (id, n) in &buckets[bi] {
            if *n == 0 {
                continue;
            }
            let s1 = id * i * (n) + id * ((n) * (n - 1) / 2);
            sum += s1;
            i += n;
        }
        i += spaces[bi].1;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"2333133121414131402"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 2858);
    }
}
