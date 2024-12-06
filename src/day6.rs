use std::collections::HashSet;

use aoc_runner_derive::aoc;

type Input<'a> = (usize, usize, usize, usize, &'a [u8]);
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn parse(input: &str) -> Input {
    let width = input.lines().next().unwrap().len() + 1;
    let height = input.lines().count();
    let input = input.as_bytes();
    let mut gi = 0;
    for (ci, &c) in input.iter().enumerate() {
        if c == b'^' {
            gi = ci;
            break;
        }
    }
    let gd = match input[gi] {
        b'^' => 0,
        b'>' => 1,
        b'v' => 2,
        b'<' => 3,
        _ => unreachable!(),
    };
    (width, height, gi, gd, input)
}

fn idx(width: usize, x: isize, y: isize) -> usize {
    if x < 0 || y < 0 {
        return usize::MAX;
    }
    x as usize + y as usize * width
}

fn guard_run(input: &Input) -> HashSet<(isize, isize)> {
    let (width, height, gi, mut gd, input) = input;
    let mut visited = HashSet::new();
    let mut x = (gi % width) as isize;
    let mut y = (gi / width) as isize;

    loop {
        let dir = DIRECTIONS[gd];
        while x + dir.0 >= 0
            && x + dir.0 < (*width as isize - 1)
            && y + dir.1 >= 0
            && y + dir.1 < *height as isize
            && (input[idx(*width, x + dir.0, y + dir.1)] == b'.'
                || idx(*width, x + dir.0, y + dir.1) == *gi)
        {
            let dir = DIRECTIONS[gd];
            visited.insert((x, y));
            x += dir.0;
            y += dir.1;
        }

        match input.get(idx(*width, x + dir.0, y + dir.1)) {
            Some(b'#') => {
                gd = (gd + 1) % 4;
            }
            Some(b'\n') | None => {
                visited.insert((x, y));
                break;
            }
            Some(_) => {
                unreachable!();
            }
        }
    }
    visited
}

fn guard_loop(input: &Input) -> bool {
    let (width, height, gi, gd, input) = input;
    #[derive(PartialEq, Eq, Clone, Copy)]
    struct State(isize, isize, usize);

    let step = |state: &mut State| -> bool {
        let dir = DIRECTIONS[state.2];
        while state.0 + dir.0 >= 0
            && state.0 + dir.0 < (*width as isize - 1)
            && state.1 + dir.1 >= 0
            && state.1 + dir.1 < *height as isize
            && (input[idx(*width, state.0 + dir.0, state.1 + dir.1)] == b'.'
                || idx(*width, state.0 + dir.0, state.1 + dir.1) == *gi)
        {
            let dir = DIRECTIONS[state.2];
            state.0 += dir.0;
            state.1 += dir.1;
        }

        match input.get(idx(*width, state.0 + dir.0, state.1 + dir.1)) {
            Some(b'#') => {
                state.2 = (state.2 + 1) % 4;
            }
            Some(b'\n') | None => {
                return true;
            }
            Some(_) => {
                unreachable!();
            }
        }
        false
    };

    let mut tortoise = State((gi % width) as isize, (gi / width) as isize, *gd);
    let mut hare = tortoise;

    loop {
        step(&mut tortoise);
        if step(&mut hare) {
            return false;
        }
        if step(&mut hare) {
            return false;
        }
        if tortoise == hare {
            return true;
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let input = parse(input);
    let visited = guard_run(&input);
    visited.len() as u32
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let input = parse(input);
    let visited = guard_run(&input);
    let (width, height, gi, gd, input) = input;
    let mut input = input.to_vec();
    let mut count = 0;
    let mut old = None;
    for (base_x, base_y) in visited {
        let ngi = idx(width, base_x, base_y);
        if ngi == gi {
            continue;
        }
        if let Some(old) = old {
            input[old] = b'.';
        }
        input[ngi] = b'#';
        //let (_, l) = guard_run(&(width, height, gi, gd, &input));
        let l = guard_loop(&(width, height, gi, gd, &input));
        if l {
            count += 1;
        }
        old = Some(ngi);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
