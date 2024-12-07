use gxhash::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(usize, Vec<usize>)>;

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Input {
    let mut r = Vec::new();
    for l in input.lines() {
        let mut s = l.split(" ");
        let res = s.next().unwrap();
        let res = res[..res.len() - 1].parse::<usize>().unwrap();

        let mut numbers = Vec::new();
        for s1 in s {
            numbers.push(s1.parse::<usize>().unwrap());
        }
        r.push((res, numbers))
    }
    r
}

#[allow(dead_code)]
fn dp(
    ci: usize,
    sum: usize,
    target: usize,
    numbers: &[usize],
    part2: bool,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(v) = cache.get(&(ci, sum)) {
        return *v;
    }
    if ci >= numbers.len() {
        let v = (sum == target) as usize;
        cache.insert((ci, sum), v);
        return v;
    }
    if sum > target {
        cache.insert((ci, sum), 0);
        return 0;
    }

    let num = numbers[ci];
    if ci == 0 {
        let res = dp(ci + 1, num, target, numbers, part2, cache);
        cache.insert((ci, sum), res);
        return res;
    }

    let mut res = 0;

    // check '+'
    res += dp(ci + 1, sum + num, target, numbers, part2, cache);

    // check '*'
    res += dp(ci + 1, sum * num, target, numbers, part2, cache);

    // check '||'
    if part2 {
        let conc = format!("{}{}", sum, num);
        let conc = conc.parse::<usize>().unwrap();
        res += dp(ci + 1, conc, target, numbers, part2, cache);
    }

    cache.insert((ci, sum), res);
    res
}

#[allow(dead_code)]
fn brute1(target: usize, numbers: &[usize], part2: bool) -> bool {
    let mut stack = vec![Vec::with_capacity(100); numbers.len()];
    stack[0].push(numbers[0]);
    for i in 1..numbers.len() {
        let num = numbers[i];
        if stack[i - 1].is_empty() {
            return false;
        }
        while let Some(val) = stack[i - 1].pop() {
            // check '+'
            if val + num <= target {
                stack[i].push(val + num);
            }

            // check '*'
            if val * num <= target {
                stack[i].push(val * num);
            }

            // check '||'
            if part2 {
                let conc: usize = 10_usize.pow(num.ilog(10) + 1) * val + num;
                if conc <= target {
                    stack[i].push(conc);
                }
            }
        }
    }
    stack[numbers.len() - 1].contains(&target)
}

fn brute2(target: usize, numbers: &[usize], part2: bool) -> bool {
    let mut stack = vec![Vec::with_capacity(100); numbers.len() + 1];
    stack[numbers.len()].push(target);
    for i in (0..numbers.len()).rev() {
        let num = numbers[i];
        if stack[i + 1].is_empty() {
            return false;
        }
        while let Some(val) = stack[i + 1].pop() {
            // check '+'
            if ((val as isize) - (num as isize)) >= 0 {
                stack[i].push(val - num);
            }

            // check '*'
            if val % num == 0 {
                stack[i].push(val / num);
            }

            // check '||'
            if part2 {
                let v = val % 10_usize.pow(num.ilog(10) + 1);
                if v == num {
                    stack[i].push(val / 10_usize.pow(num.ilog(10) + 1));
                }
            }
        }
    }
    stack[0].contains(&0)
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    let mut sum = 0;
    for (target, numbers) in input {
        if brute2(*target, numbers, false) {
            sum += target;
        }
        /*
        let mut cache = HashMap::new();
        let res = dp(0, numbers[0], *target, numbers, false, &mut cache);
        if res > 0 {
            sum += target;
        }
        */
    }
    sum
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    let mut sum: usize = 0;
    for (target, numbers) in input {
        if brute2(*target, numbers, true) {
            sum += target;
        }
        /*
        let mut cache = HashMap::new();
        let res = dp(0, numbers[0], *target, numbers, true, &mut cache);
        if res > 0 {
            sum = sum.checked_add(*target).unwrap();
        }
        */
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 11387);
    }
}
