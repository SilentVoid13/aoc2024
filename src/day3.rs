use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Token>;

#[derive(Debug)]
pub enum Token {
    Mul(u32, u32),
    Do,
    Dont,
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();

    let mut i = 0;
    while i < input.len() {
        let s = &input[i..];
        if s.starts_with("mul(") {
            i += 4;
            let s = &input[i..];
            let Some((content, _)) = s.split_once(")") else {
                continue;
            };
            if let Some((a, b)) = content.split_once(",") {
                let Ok(a) = a.parse() else {
                    continue;
                };
                let Ok(b) = b.parse() else {
                    continue;
                };
                i += content.len() + 1;
                res.push(Token::Mul(a, b))
            }
        } else if s.starts_with("do()") {
            i += 4;
            res.push(Token::Do);
        } else if s.starts_with("don't()") {
            i += 7;
            res.push(Token::Dont);
        } else {
            i += 1;
        }
    }
    res
}

#[aoc(day3, part1)]
pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|t| if let Token::Mul(a, b) = t { a * b } else { 0 })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Input) -> u32 {
    let mut enabled = true;
    let mut sum = 0;
    for t in input {
        match t {
            Token::Do => enabled = true,
            Token::Dont => enabled = false,
            Token::Mul(a, b) if enabled => sum += a * b,
            _ => {}
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const EXAMPLE2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2)), 48);
    }
}
