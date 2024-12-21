use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{Point, DOWN, LEFT, ORTHO, RIGHT, UP},
};
use gxhash::{HashMap, HashMapExt, HashSet, HashSetExt};

type Input = (Vec<(String, usize)>, [ReachMap; 2]);
type ReachMap = HashMap<char, HashMap<char, Vec<String>>>;

#[inline]
fn dir_to_char(d: Point) -> char {
    match d {
        UP => '^',
        DOWN => 'v',
        LEFT => '<',
        RIGHT => '>',
        _ => unreachable!(),
    }
}

pub fn compute_reachmap(keypad: &Grid<char>) -> ReachMap {
    let mut reachmap: ReachMap = HashMap::new();
    for ci in 0..keypad.bytes.len() {
        let mut keys: HashMap<char, Vec<String>> = HashMap::new();
        let p = keypad.as_point(ci);
        // HACK: hacky way to designate invalid points for the non-rectangular grid
        if keypad[p] == 'X' {
            continue;
        }
        keys.insert(keypad[p], vec!["A".to_string()]);

        // BFS to all neighbors
        let mut q = VecDeque::new();
        q.push_back((p, vec![p], String::new()));
        let mut weights = Grid {
            width: keypad.width,
            height: keypad.height,
            bytes: vec![usize::MAX; keypad.width * keypad.height],
        };
        while let Some((p, path, dir_path)) = q.pop_front() {
            for dir in ORTHO {
                let new_p = p + dir;
                if keypad.contains(new_p)
                    && keypad[new_p] != 'X'
                    && !path.contains(&new_p)
                    && path.len() < weights[new_p]
                {
                    let mut new_path = path.clone();
                    new_path.push(new_p);
                    weights[new_p] = new_path.len();

                    let mut new_dir_path = dir_path.clone();
                    new_dir_path.push(dir_to_char(dir));

                    let mut np = new_dir_path.clone();
                    np.push('A');
                    keys.entry(keypad[new_p]).or_default().push(np);

                    q.push_back((new_p, new_path, new_dir_path));
                }
            }
        }
        reachmap.insert(keypad[p], keys);
    }
    reachmap
}

#[aoc_generator(day21)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let num: usize = l[..3].parse().unwrap();
        res.push((l.to_string(), num));
    }

    let bytes = vec!['7', '8', '9', '4', '5', '6', '1', '2', '3', 'X', '0', 'A'];
    let keypad1 = Grid {
        width: 3,
        height: 4,
        bytes,
    };
    let keypad1 = compute_reachmap(&keypad1);

    let bytes = vec!['X', '^', 'A', '<', 'v', '>'];
    let keypad2 = Grid {
        width: 3,
        height: 2,
        bytes,
    };
    let keypad2 = compute_reachmap(&keypad2);

    (res, [keypad1, keypad2])
}

fn reachmap_dfs(reachmap: &ReachMap, c: char, value: &str) -> HashSet<String> {
    let mut q = VecDeque::new();
    q.push_back((String::new(), c, 0));
    let mut res = HashSet::new();
    while let Some((s, c, i)) = q.pop_front() {
        if i == value.len() {
            res.insert(s);
            continue;
        }
        let target_c = value.chars().nth(i).unwrap();
        for path in &reachmap[&c][&target_c] {
            let mut new_s = s.clone();
            new_s += path;
            q.push_back((new_s, target_c, i + 1));
        }
    }
    res
}

fn dp<'a>(
    reachmap: &'a ReachMap,
    path: &'a str,
    depth: usize,
    target_depth: usize,
    cache: &mut HashMap<(&'a str, usize), usize>,
) -> usize {
    if let Some(&res) = cache.get(&(path, depth)) {
        return res;
    }
    if depth == target_depth {
        cache.insert((path, depth), path.len());
        return path.len();
    }

    let mut res = 0;

    // NOTE: we know that the first character is always 'A' for any "sub-slice"
    // for the very first slice, the robot starts from 'A'
    // for other slices, we know we had to press the button with 'A' as the final step for the preceding slice
    let mut cur_c = 'A';
    for c in path.chars() {
        let mut sub_res = usize::MAX;
        for subpath in &reachmap[&cur_c][&c] {
            let r = dp(reachmap, subpath, depth + 1, target_depth, cache);
            sub_res = sub_res.min(r);
        }
        res += sub_res;

        // NOTE: the "sub-robot" will leave from the button we just asked it to press
        cur_c = c;
    }

    cache.insert((path, depth), res);
    res
}

fn solve(input: &Input, steps: usize) -> usize {
    let (values, [keypad1, keypad2]) = input;
    let mut sum = 0;
    for (base_str, num) in values {
        let base_paths = reachmap_dfs(keypad1, 'A', base_str);
        let mut best = usize::MAX;
        for base_path in base_paths {
            let r = dp(keypad2, &base_path, 0, steps, &mut HashMap::new());
            best = best.min(r);
        }
        sum += best * num;
    }
    sum
}

#[aoc(day21, part1)]
pub fn part1(input: &Input) -> usize {
    solve(input, 2)
}

#[aoc(day21, part2)]
pub fn part2(input: &Input) -> usize {
    solve(input, 25)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 154115708116294);
    }
}
