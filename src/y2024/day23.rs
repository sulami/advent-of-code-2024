use crate::print_results;
use ahash::{HashMap, HashMapExt, HashSet};
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/23");
    let connections = build_connections(input);
    print_results(
        2024,
        23,
        part_1(&connections),
        part_2(&connections),
        Some(start),
    );
}

fn part_1(connections: &HashMap<&str, Vec<&str>>) -> usize {
    connections
        .iter()
        .filter(|(_, vs)| vs.len() >= 2)
        .flat_map(|(k, v)| {
            v.iter()
                .tuple_combinations()
                .filter(|&(a, b)| connections[a].contains(b))
                .map(move |(a, b)| [k, a, b].into_iter().sorted_unstable().collect_vec())
        })
        .filter(|cs| cs.iter().any(|c| c.starts_with("t")))
        .unique()
        .count()
}

fn part_2(connections: &HashMap<&str, Vec<&str>>) -> String {
    largest_cluster(connections)
        .iter()
        .sorted_unstable()
        .join(",")
}

fn largest_cluster<'a>(connections: &HashMap<&'a str, Vec<&'a str>>) -> HashSet<&'a str> {
    let mut largest_so_far = HashSet::default();
    for (&k, peers) in connections.iter() {
        if largest_so_far.contains(k) {
            continue;
        }
        let groups = peers
            .iter()
            .powerset()
            .filter(|group| group.len() >= largest_so_far.len())
            .collect_vec();
        let best_group = groups.into_iter().rev().find(|group| {
            group
                .iter()
                .tuple_combinations()
                .all(|(a, b)| connections[*a].contains(b))
        });
        if let Some(group) = best_group {
            largest_so_far = HashSet::from_iter([k]);
            largest_so_far.extend(group);
        }
    }
    largest_so_far
}

fn build_connections(input: &str) -> HashMap<&str, Vec<&str>> {
    input.lines().map(parse_connection).fold(
        HashMap::new(),
        |mut acc: HashMap<&str, Vec<&str>>, (a, b)| {
            acc.entry(a).or_default().push(b);
            acc.entry(b).or_default().push(a);
            acc
        },
    )
}

fn parse_connection(s: &str) -> (&str, &str) {
    (&s[0..2], &s[3..5])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_connection() {
        assert_eq!(parse_connection("fo-ba"), ("fo", "ba"));
    }

    const INPUT: &str = "\
kh-tc
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
td-yn";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&build_connections(INPUT)), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&build_connections(INPUT)), "co,de,ka,ta");
    }
}
