use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};
use gxhash::{HashSet, HashSetExt};

type Input = (Grid<u8>, Vec<Point>);

#[aoc_generator(day15)]
pub fn parse(input: &str) -> Input {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = Grid::parse(grid);
    let moves = moves
        .trim()
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '<' => LEFT,
            '>' => RIGHT,
            '^' => UP,
            'v' => DOWN,
            _ => panic!("unknown char {:?}", c),
        })
        .collect();
    (grid, moves)
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> usize {
    let (grid, moves) = input;
    let mut grid = grid.clone();
    let mut cur_p = grid.find(b'@').unwrap();
    let mut mv_points = Vec::new();
    for &dir in moves {
        mv_points.clear();
        mv_points.push(cur_p);
        let mut p = cur_p;
        while grid.contains(p + dir) && grid[dir + p] == b'O' {
            mv_points.push(p + dir);
            p += dir;
        }
        if grid[p + dir] == b'#' {
            continue;
        }
        for mv_point in mv_points.iter_mut().rev() {
            let old = grid[*mv_point];
            grid[*mv_point] = b'.';
            grid[*mv_point + dir] = old;
            *mv_point += dir;
        }
        cur_p = mv_points[0];
    }
    let mut sum = 0;
    for bx in grid.find_all(b'O') {
        sum += (100 * bx.y + bx.x) as usize;
    }
    sum
}

fn expand_grid(grid: &Grid<u8>) -> Grid<u8> {
    let mut new_bytes = vec![];
    for b in grid.bytes.iter() {
        match b {
            b'.' => new_bytes.extend(b".."),
            b'#' => new_bytes.extend(b"##"),
            b'O' => new_bytes.extend(b"[]"),
            b'@' => new_bytes.extend(b"@."),
            _ => panic!("unknown byte {:?}", b),
        }
    }
    Grid {
        width: grid.width * 2,
        height: grid.height,
        bytes: new_bytes,
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> usize {
    let (grid, moves) = input;
    let mut grid = expand_grid(grid);
    let mut cur_p = grid.find(b'@').unwrap();
    let mut mv_points = Vec::new();
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    for &dir in moves {
        mv_points.clear();
        seen.clear();
        q.clear();

        mv_points.push(cur_p);
        q.push_front(cur_p);
        let mut blocked = false;
        while let Some(mut p) = q.pop_front() {
            if grid[dir + p] == b'[' || grid[dir + p] == b']' {
                p += dir;
                if seen.insert(p) {
                    q.push_back(p);
                    mv_points.push(p);
                }
                if grid[p] == b'[' && seen.insert(p + RIGHT) {
                    q.push_back(p + RIGHT);
                    mv_points.push(p + RIGHT);
                } else if grid[p] == b']' && seen.insert(p + LEFT) {
                    q.push_back(p + LEFT);
                    mv_points.push(p + LEFT);
                }
            }
            if grid[p + dir] == b'#' {
                blocked = true;
                break;
            }
        }
        if blocked {
            continue;
        }
        for mv_point in mv_points.iter_mut().rev() {
            let old = grid[*mv_point];
            grid[*mv_point] = b'.';
            grid[*mv_point + dir] = old;
            *mv_point += dir;
        }
        cur_p = mv_points[0];
    }
    let mut sum = 0;
    for bx in grid.find_all(b'[') {
        sum += (100 * bx.y + bx.x) as usize;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 9021);
    }
}
