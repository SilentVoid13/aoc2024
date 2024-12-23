use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashMap, HashMapExt};
use itertools::Itertools;

type Input = (Vec<Vec<usize>>, HashMap<usize, String>);

#[aoc_generator(day23)]
pub fn parse(input: &str) -> Input {
    let mut name_i = HashMap::new();
    let mut i_name = HashMap::new();
    let mut name_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in input.lines() {
        let (name, neigh) = l.split_once('-').unwrap();
        if !name_i.contains_key(name) {
            let id = name_i.len();
            name_i.insert(name, id);
            i_name.insert(id, name.to_string());
        }
        name_map.entry(name).or_default().push(neigh);
        name_map.entry(neigh).or_default().push(name);
    }

    // build the adjacency list graph
    let mut graph = vec![Vec::new(); name_i.len()];
    for (name, neighs) in &name_map {
        for neigh in neighs {
            graph[name_i[name]].push(name_i[neigh]);
        }
    }
    (graph, i_name)
}

#[aoc(day23, part1)]
pub fn part1(input: &Input) -> usize {
    let (graph, i_name) = input;
    let mut sum = 0;
    let mut seen = vec![false; graph.len()];
    for i in 0..graph.len() {
        seen[i] = true;
        for ns in graph[i].iter().combinations(2) {
            let n1 = ns[0];
            let n2 = ns[1];
            // if we visited one of the nodes, the clique has already been found
            if seen[*n1] || seen[*n2] {
                continue;
            }
            if graph[*n1].contains(n2)
                && (i_name[&i].starts_with("t")
                    || i_name[n1].starts_with("t")
                    || i_name[n2].starts_with("t"))
            {
                sum += 1;
            }
        }
    }
    sum
}

#[aoc(day23, part2)]
pub fn part2(input: &Input) -> String {
    let (graph, i_name) = input;
    let mut best = 0;
    let mut s_best = String::new();
    for node in 0..graph.len() {
        let mut q = VecDeque::new();
        let mut cur_clique = vec![node];
        q.push_back(node);
        while let Some(node) = q.pop_front() {
            if cur_clique.len() > best {
                best = cur_clique.len();
                let mut names = cur_clique.iter().map(|i| i_name[i].as_str()).collect_vec();
                names.sort();
                s_best = names.join(",");
            }
            'n: for &neigh in &graph[node] {
                for c in &cur_clique {
                    if !graph[neigh].contains(c) {
                        continue 'n;
                    }
                }
                cur_clique.push(neigh);
                q.push_back(neigh);
            }
        }
    }
    s_best
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "co,de,ka,ta");
    }
}
