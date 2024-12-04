use aoc_runner_derive::{aoc, aoc_generator};

type Input = (String, usize, usize);

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Input {
    let mut input = input.to_string();
    // little trick to deal with the missing final newline
    input.push('\n');
    let width = input.lines().next().unwrap().len() + 1;
    let height = input.lines().count();
    (input, width, height)
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> u32 {
    let word = b"XMAS";
    let (input, width, height) = input;
    let input = input.as_bytes();

    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut count = 0;
    for ci in 0..input.len() {
        if input[ci] == b'X' {
            for direction in DIRECTIONS {
                let mut row_i = (ci % width) as isize;
                let mut col_i = (ci / width) as isize;
                let mut wi = 0;
                while wi < word.len()
                    && col_i >= 0
                    && col_i < *height as isize
                    && row_i >= 0
                    && row_i < *width as isize
                    && input[col_i as usize * width + row_i as usize] == word[wi]
                {
                    col_i += direction.1;
                    row_i += direction.0;
                    wi += 1;
                }
                if wi == word.len() {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> u32 {
    let mut count = 0;
    let (input, width, height) = input;
    let input = input.as_bytes();
    for ci in 0..input.len() {
        let row_i = ci % width;
        let col_i = ci / width;
        if row_i > 0
            && col_i > 0
            && row_i < width - 1
            && col_i < height - 1
            && input[ci] == b'A'
            && ([input[ci - width - 1], input[ci + width + 1]] == [b'M', b'S']
                || [input[ci - width - 1], input[ci + width + 1]] == [b'S', b'M'])
            && ([input[ci + width - 1], input[ci - width + 1]] == [b'M', b'S']
                || [input[ci + width - 1], input[ci - width + 1]] == [b'S', b'M'])
        {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 9);
    }
}
