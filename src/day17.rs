use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = ([usize; 3], Vec<(u8, u8)>);
const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[aoc_generator(day17)]
pub fn parse(input: &str) -> Input {
    let mut final_regs = [0; 3];
    let (regs, instructions) = input.split_once("\n\n").unwrap();
    for (i, r) in regs
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse().unwrap())
        .enumerate()
    {
        final_regs[i] = r;
    }
    let (_, instructions) = instructions.split_once(": ").unwrap();
    let instructions = instructions
        .split(",")
        .tuples()
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    (final_regs, instructions)
}

#[inline]
fn combo(v: u8, regs: &[usize]) -> usize {
    match v {
        0..4 => v as usize,
        i @ 4..7 => regs[(i - 4) as usize],
        _ => unreachable!(),
    }
}

pub fn run(regs: &mut [usize; 3], instructions: &[(u8, u8)]) -> Vec<u8> {
    let mut pc = 0;
    let mut s = Vec::with_capacity(100);
    while pc < instructions.len() {
        let (opcode, operand) = instructions[pc];
        match opcode {
            0 => {
                // a = a / 2^operand
                let num = regs[A];
                regs[A] = num >> combo(operand, regs);
                pc += 1;
            }
            1 => {
                // b ^= operand
                regs[B] ^= operand as usize;
                pc += 1;
            }
            2 => {
                // b = operand % 8
                regs[B] = combo(operand, regs) & 7;
                pc += 1;
            }
            3 => {
                // if a != 0: jump to operand
                if regs[A] != 0 {
                    pc = operand as usize;
                } else {
                    pc += 1;
                }
            }
            4 => {
                // b ^= c
                regs[B] ^= regs[C];
                pc += 1;
            }
            5 => {
                // print operand
                let r = combo(operand, regs) & 7;
                s.push(r as u8);
                pc += 1;
            }
            6 => {
                // b = a / 2^operand
                let num = regs[A];
                regs[B] = num >> combo(operand, regs);
                pc += 1;
            }
            7 => {
                // c = a / 2^operand
                let num = regs[A];
                regs[C] = num >> combo(operand, regs);
                pc += 1;
            }
            _ => unimplemented!(),
        }
    }
    s
}

#[aoc(day17, part1)]
pub fn part1(input: &Input) -> String {
    let mut regs = input.0;
    let s = run(&mut regs, &input.1);
    s.iter().join(",")
}

fn solve(input: &Input, output: &[u8]) -> Vec<usize> {
    if output.is_empty() {
        return vec![0];
    }
    let mut cands = vec![];
    for a_cand in solve(input, &output[1..]) {
        for aa in 0..8 {
            let a = (a_cand << 3) + aa;
            let mut regs = [a, 0, 0];
            let s = run(&mut regs, &input.1);
            if s[0] == output[0] && !cands.contains(&aa) {
                cands.push(a);
            }
        }
    }
    cands
}

#[aoc(day17, part2)]
pub fn part2(input: &Input) -> usize {
    // part 2 requires reverse engineering our input program
    //
    // my program does the following until 'a' reaches 0:
    // b = a & 7
    // b ^= 7
    // c = a >> b
    // b ^= 7
    // b ^= c
    // a = a >> 3
    // print (b & 7)
    //
    // it takes each digit of 'a' in base 8, performs some operations on it, and prints the final
    // program digit
    // we notice that the operations only depend on the lower bits of 'a' (c = a >> b)
    // this means we can start bruteforcing from the last digit of 'a' and work our way up
    // a non-terminal recursive function is perfect for this use case
    // we will be able to prune a lot of invalid paths as we go up

    let mut flat_i = Vec::new();
    for &(opcode, operand) in &input.1 {
        flat_i.push(opcode);
        flat_i.push(operand);
    }
    let r = solve(input, &flat_i);
    *r.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const EXAMPLE2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    const EXAMPLE3: &str = r#"Register A: 33024962
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2)), 117440);
        assert_eq!(part2(&parse(EXAMPLE3)), 216584205979245);
    }
}
