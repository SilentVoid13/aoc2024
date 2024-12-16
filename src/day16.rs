use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT},
};
use gxhash::{HashSet, HashSetExt};

type Input = (Grid<u8>, Point, Point);

#[aoc_generator(day16)]
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    (grid, start, end)
}

#[inline]
fn di(d: Point) -> usize {
    if d == RIGHT {
        0
    } else if d == LEFT {
        1
    } else if d == DOWN {
        2
    } else {
        3
    }
}

fn dijkstra(input: &Input) -> usize {
    let (grid, start, end) = input;

    let idx =
        |p: Point, d: Point| -> usize { p.y as usize * grid.width * 4 + p.x as usize * 4 + di(d) };
    let mut weights = vec![usize::MAX; grid.bytes.len() * 4];

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, *start, RIGHT)));

    while let Some(e) = q.pop() {
        let (score, pos, cur_dir) = e.0;
        if pos == *end {
            return score;
        }

        let new_dirs = [cur_dir.clockwise(), cur_dir, cur_dir.counter_clockwise()];
        for dir in new_dirs {
            let next = pos + dir;
            let new_score = if dir == cur_dir {
                score + 1
            } else {
                score + 1001
            };
            if grid[next] != b'#' && new_score < weights[idx(next, dir)] {
                weights[idx(next, dir)] = new_score;
                q.push(Reverse((new_score, next, dir)));
            }
        }
    }
    panic!("No path found");
}

fn dijkstra_all_paths(input: &Input) -> usize {
    let (grid, start, end) = input;
    let idx =
        |p: Point, d: Point| -> usize { p.y as usize * grid.width * 4 + p.x as usize * 4 + di(d) };
    let mut weights = vec![usize::MAX; grid.bytes.len() * 4];
    let mut prevs = vec![vec![]; grid.bytes.len() * 4];

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, *start, RIGHT)));
    let mut ends = HashSet::new();
    let mut best_end = usize::MAX;

    while let Some(e) = q.pop() {
        let (score, pos, cur_dir) = e.0;

        if pos == *end {
            if score < best_end {
                ends.insert((pos, cur_dir));
                best_end = score;
                continue;
            } else {
                break;
            }
        }

        let new_dirs = [cur_dir.clockwise(), cur_dir, cur_dir.counter_clockwise()];
        for dir in new_dirs {
            let next = pos + dir;
            let new_score = if dir == cur_dir {
                score + 1
            } else {
                score + 1001
            };
            if grid[next] != b'#' && new_score <= weights[idx(next, dir)] {
                weights[idx(next, dir)] = new_score;
                prevs[idx(next, dir)].retain(|(s, _)| *s == new_score);
                prevs[idx(next, dir)].push((new_score, (pos, cur_dir)));
                // NOTE: we have to do that because rust's BinaryHeap doesn't support
                // the decrease_key operation and we don't want to accumulate duplicates
                q.retain(|e| e.0 .1 != next || e.0 .2 != dir);
                q.push(Reverse((new_score, next, dir)));
            }
        }
    }

    // count distinct squares
    let mut q = vec![];
    let mut seen = HashSet::new();
    let mut h = HashSet::new();
    for e in ends {
        seen.insert(e);
        q.push(e);
    }
    while let Some((pos, cur_dir)) = q.pop() {
        h.insert(pos);
        for (_, prev) in &prevs[idx(pos, cur_dir)] {
            if seen.insert(*prev) {
                q.push(*prev);
            }
        }
    }
    h.len()
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> usize {
    dijkstra(input)
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> usize {
    dijkstra_all_paths(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const EXAMPLE2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7036);
        assert_eq!(part1(&parse(EXAMPLE2)), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 45);
        assert_eq!(part2(&parse(EXAMPLE2)), 64);
    }
}
