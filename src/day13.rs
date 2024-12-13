use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::point::Point;
use gxhash::HashMap;

type Input = Vec<(Point, Point, Point)>;

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for s in input.split("\n\n") {
        let mut points = Vec::new();
        for l in s.lines() {
            let vals = l.split(['+', '=']);
            let mut coords = [0; 2];
            for (i, val) in vals.skip(1).enumerate() {
                let val = val.split(",").next().unwrap();
                let val = val.parse().unwrap();
                coords[i] = val;
            }
            points.push(Point::new(coords[0], coords[1]));
        }
        res.push((points[0], points[1], points[2]));
    }
    res
}

// naive solutionl
#[allow(dead_code)]
fn dp(
    cur: Point,
    target: Point,
    ba: Point,
    bb: Point,
    na: usize,
    nb: usize,
    cost_a: usize,
    cost_b: usize,
    max_press: usize,
    cache: &mut HashMap<(Point, usize, usize), usize>,
) -> usize {
    if let Some(&v) = cache.get(&(cur, na, nb)) {
        return v;
    }
    if cur.x > target.x || cur.y > target.y {
        return 0;
    }
    if na > max_press || nb > max_press {
        return 0;
    }
    if cur == target {
        return na * cost_a + nb * cost_b;
    }
    let mut best = 0;

    // check a
    best = best.max(dp(
        cur + ba,
        target,
        ba,
        bb,
        na + 1,
        nb,
        cost_a,
        cost_b,
        max_press,
        cache,
    ));

    // check b
    best = best.max(dp(
        cur + bb,
        target,
        ba,
        bb,
        na,
        nb + 1,
        cost_a,
        cost_b,
        max_press,
        cache,
    ));

    cache.insert((cur, na, nb), best);
    best
}

fn lin_solve(a: Point, b: Point, p: Point) -> (i64, i64) {
    let na = (p.x * b.y - p.y * b.x) / (a.x * b.y - b.x * a.y);
    let nb = (p.y * a.x - p.x * a.y) / (a.x * b.y - b.x * a.y);
    (na, nb)
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    let cost_a = 3;
    let cost_b = 1;
    let mut sum = 0;
    for (a, b, p) in input {
        /*
        let max_press = 100;
        let r = dp(
            Point::new(0, 0),
            *p,
            *a,
            *b,
            0,
            0,
            cost_a,
            cost_b,
            max_press,
            &mut HashMap::new(),
        );
        sum += r;
        */
        let (na, nb) = lin_solve(*a, *b, *p);
        let rx = a.x * na + b.x * nb;
        let ry = a.y * na + b.y * nb;
        if rx == p.x && ry == p.y {
            sum += na as usize * cost_a + nb as usize * cost_b;
        }
    }
    sum
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> usize {
    let cost_a = 3;
    let cost_b = 1;

    let mut sum = 0;
    for (a, b, mut p) in input {
        p += Point::new(10000000000000, 10000000000000);
        let (na, nb) = lin_solve(*a, *b, p);
        let rx = a.x * na + b.x * nb;
        let ry = a.y * na + b.y * nb;
        if rx == p.x && ry == p.y {
            sum += na as usize * cost_a + nb as usize * cost_b;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 875318608908);
    }
}
