use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{Point, DIAG, ORTHO},
};

type Input = Grid<u8>;

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> usize {
    let grid = input;
    let mut sum = 0;
    let mut seen = grid.copy_filled(0);
    for (ci, c) in grid.bytes.iter().copied().enumerate() {
        let p = grid.as_point(ci);
        if seen[p] != 0 {
            continue;
        }
        seen[p] = 1;
        let mut q = VecDeque::new();
        let mut area = 1;
        let mut perim = 0;
        q.push_back((p, c));
        while let Some((p, cur_c)) = q.pop_front() {
            for dir in ORTHO {
                if grid.contains(p + dir) && grid[p + dir] == cur_c {
                    if seen[p + dir] == 0 {
                        seen[p + dir] = 1;
                        area += 1;
                        q.push_back((p + dir, cur_c));
                    }
                } else {
                    perim += 1;
                }
            }
        }
        sum += area * perim;
    }
    sum
}

#[aoc(day12, part2)]
pub fn part2(input: &Input) -> usize {
    let grid = input;
    let mut seen = grid.copy_filled(0);
    let mut sum = 0;
    for (ci, c) in grid.bytes.iter().copied().enumerate() {
        let p = grid.as_point(ci);
        if seen[p] != 0 {
            continue;
        }
        seen[p] = 1;
        let mut q = VecDeque::new();
        let mut area = 1;
        q.push_back((p, c));
        let mut corners = 0;
        while let Some((p, cur_c)) = q.pop_front() {
            let mut neighs = vec![];
            for &dir in ORTHO.iter() {
                if grid.contains(p + dir) && grid[p + dir] == cur_c {
                    neighs.push(p + dir);
                    if seen[p + dir] == 0 {
                        seen[p + dir] = 1;
                        area += 1;
                        q.push_back((p + dir, cur_c));
                    }
                }
            }

            // outside corners
            match neighs.len() {
                0 => corners += 4,
                1 => corners += 2,
                2 if neighs[0].x != neighs[1].x && neighs[0].y != neighs[1].y => {
                    corners += 1;
                }
                _ => {}
            }

            // inside corners
            // we pair each orthogonal neighbor pair with each diagonal neighbor
            // e.g. top left with (left, top) etc
            // if the diagonal is not the same as the orthogonal pair, we have a corner
            for diag in DIAG {
                let p1 = p + Point::new(diag.x, 0);
                let p2 = p + Point::new(0, diag.y);
                if grid.contains(p1)
                    && grid.contains(p2)
                    && grid[p1] == cur_c
                    && grid[p2] == cur_c
                    && grid[p + diag] != cur_c
                {
                    corners += 1;
                }
            }
        }
        sum += area * corners;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 1206);
    }
}
