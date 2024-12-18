use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{Point, ORTHO},
};

type Input = (Grid<u32>, Vec<Point>);

#[aoc_generator(day18)]
pub fn parse(input: &str) -> Input {
    let mut pos = Vec::new();
    for l in input.lines() {
        let (x, y) = l.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        pos.push(Point::new(x, y));
    }
    // test case
    let width = if pos.len() < 50 { 7 } else { 71 };
    let mut grid = Grid {
        width,
        height: width,
        bytes: vec![u32::MAX; width * width],
    };
    for (i, &pos) in pos.iter().enumerate() {
        grid[pos] = i as u32;
    }
    (grid, pos)
}

fn bfs(grid: &Grid<u32>, n_fall: u32) -> usize {
    let mut q = VecDeque::new();
    let mut seen = grid.copy_filled(0);
    let start = Point::new(0, 0);
    let end = Point::new(grid.width as i64 - 1, grid.height as i64 - 1);
    q.push_front((start, 0));
    seen[start] = 1;

    while let Some((p, score)) = q.pop_front() {
        if p == end {
            return score;
        }
        for dir in ORTHO {
            let new_p = p + dir;
            if grid.contains(new_p) && grid[new_p] > n_fall && seen[new_p] == 0 {
                seen[new_p] = 1;
                q.push_back((new_p, score + 1));
            }
        }
    }
    usize::MAX
}

#[aoc(day18, part1)]
pub fn part1(input: &Input) -> usize {
    let (grid, poss) = input;
    let sim = if poss.len() < 50 { 12 - 1 } else { 1024 - 1 };
    bfs(grid, sim)
}

#[aoc(day18, part2)]
pub fn part2(input: &Input) -> String {
    let (grid, poss) = input;
    let n_falls: Vec<_> = (0..poss.len()).collect();
    let first_block = n_falls.partition_point(|&v| bfs(grid, v as u32) != usize::MAX);
    let p = poss[first_block];
    format!("{},{}", p.x, p.y)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "6,1");
    }
}
