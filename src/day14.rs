use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{grid::Grid, point::Point};

type Input = Vec<(Point, Point)>;

#[aoc_generator(day14)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let mut points = vec![];
        for v in l.split("=").skip(1) {
            let mut s = v.split(",");
            let x = s.next().unwrap().parse().unwrap();
            let y = s
                .next()
                .unwrap()
                .split(" ")
                .next()
                .unwrap()
                .parse()
                .unwrap();
            points.push(Point::new(x, y));
        }
        res.push((points[0], points[1]));
    }
    res
}

fn fill_robots(robots: &[(Point, Point)]) -> Grid<u8> {
    let mut grid = if robots.len() < 30 {
        // test case
        Grid {
            width: 11,
            height: 7,
            bytes: vec![0; 11 * 7],
        }
    } else {
        Grid {
            width: 101,
            height: 103,
            bytes: vec![0; 101 * 103],
        }
    };
    for &(pos, _) in robots {
        grid[pos] += 1;
    }
    grid
}

fn count_quadrants(grid: &Grid<u8>) -> usize {
    let mut quadrants = [0usize; 4];
    let wm = grid.width as i64 / 2;
    let hm = grid.height as i64 / 2;
    for i in 0..grid.bytes.len() {
        let p = grid.as_point(i);
        if p.x == wm || p.y == hm {
            continue;
        }
        if grid[p] > 0 {
            quadrants[(p.y < hm) as usize * 2 + (p.x < wm) as usize] += grid[p] as usize;
        }
    }
    quadrants.iter().copied().product()
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> usize {
    let robots = input;
    let mut grid = fill_robots(robots);
    let mut robots = robots.clone();
    const STEPS: i64 = 100;
    for (pos, vel) in robots.iter_mut() {
        let mut next_pos = *pos + (*vel * STEPS);
        next_pos.x = next_pos.x.rem_euclid(grid.width as i64);
        next_pos.y = next_pos.y.rem_euclid(grid.height as i64);
        grid[*pos] -= 1;
        grid[next_pos] += 1;
    }
    count_quadrants(&grid)
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> usize {
    let robots = input;
    if robots.len() < 30 {
        // test case
        return 0;
    }
    let mut grid = fill_robots(robots);
    let mut robots = robots.clone();
    let mut i = 0;
    loop {
        for (pos, vel) in robots.iter_mut() {
            let mut next_pos = *pos + *vel;
            next_pos.x = next_pos.x.rem_euclid(grid.width as i64);
            next_pos.y = next_pos.y.rem_euclid(grid.height as i64);
            grid[*pos] -= 1;
            grid[next_pos] += 1;
            *pos = next_pos;
        }
        i += 1;

        // based on an assumption: the robots for the tree base should form a line
        const DIR: Point = Point::new(1, 0);
        let line_length = 10;
        for &(mut p, _) in &robots {
            let mut is_tree = true;
            for _ in 0..line_length {
                if !grid.contains(p) || grid[p] == 0 {
                    is_tree = false;
                    break;
                }
                p += DIR;
            }
            if is_tree {
                return i;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3 "#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
