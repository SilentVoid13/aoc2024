use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{Point, ORTHO},
};

type Input = (Grid<u8>, Point, Point);

#[aoc_generator(day20)]
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    (grid, start, end)
}

// NOTE: assumption: the maze ends with the exit
fn solve(grid: &Grid<u8>, start: Point, limit: usize, bound: usize) -> usize {
    let mut q = VecDeque::new();
    let bytes = vec![usize::MAX; grid.width * grid.height];
    let mut dist_grid = Grid {
        width: grid.width,
        height: grid.height,
        bytes,
    };
    dist_grid[start] = 0;
    q.push_back((start, 0));
    let mut path = Vec::new();

    // fist perform a full BFS to compute costs for reachable squares
    while let Some((p, score)) = q.pop_front() {
        path.push(p);
        for dir in ORTHO {
            let new_p = p + dir;
            if grid[new_p] != b'#' && dist_grid[new_p] == usize::MAX {
                dist_grid[new_p] = score + 1;
                q.push_back((new_p, score + 1));
            }
        }
    }

    let mut sum = 0;
    let limit = limit as i64;
    for p in path {
        let base_p = p;
        for off_x in -limit..=limit {
            for off_y in -limit..=limit {
                let off = Point::new(off_x, off_y);
                let new_p = base_p + off;
                if !grid.contains(new_p) || grid[new_p] == b'#' || base_p.manhattan(new_p) > limit {
                    continue;
                }
                let skipped = dist_grid[new_p]
                    .saturating_sub(dist_grid[base_p])
                    .saturating_sub(base_p.manhattan(new_p) as usize);
                if skipped >= bound {
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> usize {
    let (grid, start, _) = input;
    // for test case
    let bound = if grid.bytes.len() < 1000 { 1 } else { 100 };
    solve(grid, *start, 2, bound)
}

#[aoc(day20, part2)]
pub fn part2(input: &Input) -> usize {
    let (grid, start, _) = input;
    // for test case
    let bound = if grid.bytes.len() < 1000 { 50 } else { 100 };
    solve(grid, *start, 20, bound)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 285);
    }
}
