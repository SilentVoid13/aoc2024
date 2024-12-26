use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{grid::Grid, point::Point};

type Input = (Vec<Grid<u8>>, Vec<Grid<u8>>);

#[aoc_generator(day25)]
pub fn parse(input: &str) -> Input {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for s in input.split("\n\n") {
        let grid = Grid::parse(s);
        if grid.bytes[0] == b'#' {
            locks.push(grid);
        } else {
            keys.push(grid);
        }
    }
    (keys, locks)
}

#[aoc(day25, part1)]
pub fn part1(input: &Input) -> usize {
    let (keys, locks) = input;
    let mut sum = 0;
    for lock in locks {
        'k: for key in keys {
            for x in 0..key.width {
                let mut c1 = 0;
                let mut c2 = 0;
                for y in 1..key.height - 1 {
                    let p = Point::new(x as i64, y as i64);
                    if key[p] == b'#' {
                        c1 += 1;
                    }
                    if lock[p] == b'#' {
                        c2 += 1;
                    }
                }
                if c1 + c2 > 5 {
                    continue 'k;
                }
            }
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }
}
