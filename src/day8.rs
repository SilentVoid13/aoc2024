use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{
    grid::Grid,
    point::Point,
};
use itertools::Itertools;

type Input = (Grid<u8>, HashMap<u8, Vec<Point>>);

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut pos: HashMap<u8, Vec<Point>> = HashMap::new();
    for (ci, c) in grid.bytes.iter().copied().enumerate() {
        if c != b'.' {
            let p = grid.as_point(ci);
            pos.entry(c).or_default().push(p);
        }
    }
    (grid, pos)
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut seen = HashSet::new();
    let (grid, poss) = input;
    for pos in poss.values() {
        for p in pos.iter().combinations(2) {
            let p1 = *p[0];
            let p2 = *p[1];
            let diff = p2 - p1;
            if grid.contains(p1 - diff) {
                seen.insert(p1 - diff);
            }
            if grid.contains(p2 + diff) {
                seen.insert(p2 + diff);
            }
        }
    }
    seen.len() as u32
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> u32 {
    let mut seen = HashSet::new();
    let (grid, poss) = input;
    for pos in poss.values() {
        for p in pos.iter().combinations(2) {
            let mut p1 = *p[0];
            let mut p2 = *p[1];
            let diff = p2 - p1;
            seen.insert(p1);
            seen.insert(p2);

            let mut found = true;
            while found {
                found = false;
                if grid.contains(p1 - diff) {
                    found = true;
                    seen.insert(p1 - diff);
                    p1 -= diff;
                }
                if grid.contains(p2 + diff) {
                    found = true;
                    seen.insert(p2 + diff);
                    p2 += diff;
                }
            }
        }
    }
    seen.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }
}
