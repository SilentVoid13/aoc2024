use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashMap, HashMapExt, HashSet, HashSetExt};
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
        for n1i in 0..graph[i].len() {
            let n1 = graph[i][n1i];
            for n2i in n1i + 1..graph[i].len() {
                let n2 = graph[i][n2i];
                // if we visited one of the nodes, the clique has already been found
                if seen[n1] || seen[n2] {
                    continue;
                }
                if graph[n1].contains(&n2)
                    && (i_name[&i].starts_with("t")
                        || i_name[&n1].starts_with("t")
                        || i_name[&n2].starts_with("t"))
                {
                    sum += 1;
                }
            }
        }
    }
    sum
}

// Bron-Kerbosch algorithm, not the most efficient
#[allow(dead_code)]
fn bron_kerbosch(
    graph: &Vec<Vec<usize>>,
    r: HashSet<usize>,
    mut p: HashSet<usize>,
    mut x: HashSet<usize>,
) -> HashSet<usize> {
    if p.is_empty() && x.is_empty() {
        return r;
    }
    let mut best = HashSet::new();
    for v in p.clone() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let res = bron_kerbosch(
            graph,
            new_r,
            p.intersection(&graph[v].iter().copied().collect())
                .cloned()
                .collect(),
            x.intersection(&graph[v].iter().copied().collect())
                .cloned()
                .collect(),
        );
        if res.len() > best.len() {
            best = res;
        }
        p.remove(&v);
        x.insert(v);
    }
    best
}

#[aoc(day23, part2)]
pub fn part2(input: &Input) -> String {
    let (graph, i_name) = input;
    let mut best_clique = Vec::new();
    let mut cur_clique = Vec::new();
    let mut seen = vec![false; graph.len()];
    for (n1, neighbours) in graph.iter().enumerate() {
        if !seen[n1] {
            cur_clique.clear();
            cur_clique.push(n1);
            for &n2 in neighbours {
                if cur_clique.iter().all(|c| graph[n2].contains(c)) {
                    cur_clique.push(n2);
                    // we only need to visit one node from the clique
                    seen[n2] = true;
                }
            }

            if cur_clique.len() > best_clique.len() {
                best_clique = cur_clique.clone();
            }
        }
    }
    let mut names = best_clique.iter().map(|i| i_name[i].as_str()).collect_vec();
    names.sort();
    names.join(",")
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
