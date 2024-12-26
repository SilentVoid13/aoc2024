use std::{
    collections::BTreeSet,
    fmt::{self, Display, Formatter},
};

use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashMap, HashMapExt};
use itertools::Itertools;

type Input = (
    HashMap<String, bool>,
    Vec<(Operation, String, String, String)>,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Or,
    And,
    Xor,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operation::Or => write!(f, "OR"),
            Operation::And => write!(f, "AND"),
            Operation::Xor => write!(f, "XOR"),
        }
    }
}

#[aoc_generator(day24)]
pub fn parse(input: &str) -> Input {
    let mut values = HashMap::new();
    let (p1, p2) = input.split_once("\n\n").unwrap();
    for l in p1.lines() {
        let (name, value) = l.split_once(": ").unwrap();
        values.insert(name.to_string(), value.parse::<u8>().unwrap() != 0);
    }
    let mut operations = Vec::new();
    for l in p2.lines() {
        let (ops, res) = l.split_once(" -> ").unwrap();
        let mut s = ops.split(" ");
        let op1 = s.next().unwrap().trim();
        let op = s.next().unwrap().trim();
        let op2 = s.next().unwrap().trim();

        let op = match op {
            "OR" => Operation::Or,
            "AND" => Operation::And,
            "XOR" => Operation::Xor,
            _ => todo!(),
        };
        operations.push((op, op1.to_string(), op2.to_string(), res.to_string()));
    }
    (values, operations)
}

fn run(input: &Input) -> usize {
    let (values, operations) = input;
    let mut values = values.clone();
    let mut modif = true;
    while modif {
        modif = false;
        for (op, op1, op2, res) in operations {
            if values.contains_key(res) {
                continue;
            }
            let Some(op1) = values.get(op1).copied() else {
                continue;
            };
            let Some(op2) = values.get(op2).copied() else {
                continue;
            };
            modif = true;
            match op {
                Operation::Or => {
                    values.entry(res.clone()).or_insert(op1 || op2);
                }
                Operation::And => {
                    values.entry(res.clone()).or_insert(op1 && op2);
                }
                Operation::Xor => {
                    values.entry(res.clone()).or_insert(op1 ^ op2);
                }
            }
        }
    }
    let mut zs: Vec<(&String, &bool)> = values
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .collect();
    zs.sort();
    let mut res = 0;
    for (_, value) in zs.into_iter().rev() {
        res <<= 1;
        res |= *value as usize;
    }
    res
}

#[aoc(day24, part1)]
pub fn part1(input: &Input) -> usize {
    run(input)
}

#[aoc(day24, part2)]
pub fn part2(input: &Input) -> String {
    // we know we have a binary adder of multiple bits
    // this means we have multiple full adders each composed of a XOR, AND and OR gate that are
    // connected between each other.
    // an important simplification is that we only need to detect the incorrect connections, we
    // don't need to find the correct swaps.
    // the workflow should be the following:
    // 1. input a and b are connected to a XOR gate, the 'sum' result is directly connected to a
    //    final bit
    // 2. input a and b are connected to an AND gate, the 'carry out' result is directly connected
    //    to an OR gate
    // 3. the 'carry out' OR result is connected to the next full adder's XOR/AND gate

    let (_, operations) = input;
    let highest_z = operations
        .iter()
        .filter_map(|(_, _, _, res)| {
            if res.starts_with("z") {
                Some(res)
            } else {
                None
            }
        })
        .cloned()
        .max()
        .unwrap();
    const INS: [char; 2] = ['x', 'y'];

    let mut invalids = BTreeSet::new();
    for (op, op1, op2, res) in operations {
        // only XORs should be connected to the final bit, except for the final z (last carry bit)
        if op != &Operation::Xor && *res != highest_z && res.starts_with("z") {
            invalids.insert(res.clone());
        }

        // z should only be an output
        if op1.starts_with("z") || op2.starts_with("z") {
            invalids.insert(res.clone());
        }

        // XORs should be connected to the input bits or lead to the final bit
        if *op == Operation::Xor
            && !(INS.contains(&op1.chars().next().unwrap())
                && INS.contains(&op2.chars().next().unwrap()))
            && !res.starts_with("z")
        {
            invalids.insert(res.clone());
        }

        // OR should be connected to a XOR and AND gate
        if *op == Operation::Xor {
            for (subop, subop1, subop2, _) in operations {
                if subop != &Operation::Xor
                    && subop != &Operation::And
                    && (subop1 == res || subop2 == res)
                {
                    invalids.insert(res.clone());
                }
            }
        }

        // AND should be connected to an OR gate, except for x00
        if *op == Operation::And && op1 != "x00" && op2 != "x00" {
            for (subop, subop1, subop2, _) in operations {
                if subop != &Operation::Or && (subop1 == res || subop2 == res) {
                    invalids.insert(res.clone());
                }
            }
        }
    }
    invalids.iter().join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    const EXAMPLE2: &str = r#"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2024);
    }
}
