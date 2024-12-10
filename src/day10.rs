use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{Point, ORTHO},
};
use gxhash::{HashSet, HashSetExt};

type Input = Grid<u8>;

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

fn pathfind(grid: &Grid<u8>, start: Point, p1: bool) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((start, 0));
    seen.insert(start);

    let mut score = 0;
    while let Some((pos, height)) = queue.pop_front() {
        if height == 9 {
            score += 1;
            continue;
        }
        for dir in ORTHO {
            if grid.contains(pos + dir) {
                let nh = grid[pos + dir] - b'0';
                if nh == height + 1 {
                    if p1 && !seen.insert(pos + dir) {
                        continue;
                    }
                    queue.push_front((pos + dir, nh));
                }
            }
        }
    }
    score
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> u32 {
    let grid = input;
    let mut score = 0;
    for (bi, b) in grid.bytes.iter().enumerate() {
        if *b == b'0' {
            score += pathfind(grid, grid.as_point(bi), true);
        }
    }
    score as u32
}

#[aoc(day10, part2)]
pub fn part2(input: &Input) -> u32 {
    let grid = input;
    let mut score = 0;
    for (bi, b) in grid.bytes.iter().enumerate() {
        if *b == b'0' {
            score += pathfind(grid, grid.as_point(bi), false);
        }
    }
    score as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 81);
    }
}
