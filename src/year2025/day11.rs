use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;
pub fn main(data: &str) -> Result<(usize, usize)> {
    let input = parse_input(data);
    Ok((part1(&input), part2(&input)))
}

fn parse_input(data: &'_ str) -> Graph<'_> {
    data.lines()
        .map(|line| {
            let mut split = line.split(": ");
            let source_node = split.next().unwrap();
            let target_nodes = split.next().unwrap().split(" ").collect::<Vec<_>>();
            (source_node, target_nodes)
        })
        .collect::<HashMap<_, _>>()
}

fn part1(input: &Graph) -> usize {
    count_num_paths(input, "you", "out", HashSet::new())
}

fn part2(input: &Graph) -> usize {
    let svr_to_fft = count_num_paths(input, "svr", "fft", HashSet::from(["dac"]));
    let fft_to_dac = count_num_paths(input, "fft", "dac", HashSet::new());
    let dac_to_out = count_num_paths(input, "dac", "out", HashSet::from(["fft"]));

    let svr_to_dac = count_num_paths(input, "svr", "dac", HashSet::from(["fft"]));
    let dac_to_fft = count_num_paths(input, "dac", "fft", HashSet::new());
    let fft_to_out = count_num_paths(input, "fft", "out", HashSet::from(["dac"]));

    svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out
}

// Count the number of paths from a start node to an end node using a DP + topological sort approach.
fn count_num_paths(input: &Graph, start: &str, end: &str, avoid: HashSet<&str>) -> usize {
    let topological_sort = topological_sort(input);
    let mut paths = topological_sort
        .iter()
        .clone()
        .map(|&node| (node, 0_usize))
        .collect::<HashMap<_, _>>();
    paths.insert(start, 1_usize);

    for u in topological_sort {
        if avoid.contains(&u) {
            continue;
        }
        if paths.get(&u).unwrap_or(&0) > &0 {
            let paths_to_u = *paths.get(&u).unwrap_or(&0);
            for v in input.get(&u).unwrap_or(&Vec::<&str>::new()) {
                paths.entry(v).and_modify(|v| *v += paths_to_u);
            }
        }
    }
    *paths.get(end).unwrap_or(&0)
}

/// Perform topological sort on the graph using Kahn's algorithm.
fn topological_sort<'a>(graph: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let mut graph = graph.clone();
    let mut topological_order = Vec::new();

    let mut nodes = graph.keys().cloned().collect::<HashSet<_>>();
    nodes.extend(graph.values().flatten());

    let mut zero_indegree_nodes = nodes
        .difference(&HashSet::from(
            graph.values().cloned().flatten().collect::<HashSet<_>>(),
        ))
        .cloned()
        .collect::<Vec<_>>();

    while let Some(node) = zero_indegree_nodes.pop() {
        topological_order.push(node);

        let edges = graph.remove(&node).unwrap_or(Vec::new());
        for edge in edges {
            if !graph.values().flatten().contains(&edge) {
                zero_indegree_nodes.push(edge);
            }
        }
    }

    if !graph.is_empty() {
        panic!("Graph is not acyclic!");
    }
    topological_order
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    const EXAMPLE_PART2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE);
        assert_eq!(
            input,
            HashMap::from([
                ("aaa", vec!["you", "hhh"]),
                ("you", vec!["bbb", "ccc"]),
                ("bbb", vec!["ddd", "eee"]),
                ("ccc", vec!["ddd", "eee", "fff"]),
                ("ddd", vec!["ggg"]),
                ("eee", vec!["out"]),
                ("fff", vec!["out"]),
                ("ggg", vec!["out"]),
                ("hhh", vec!["ccc", "fff", "iii"]),
                ("iii", vec!["out"])
            ])
        );
    }

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE_PART2);
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_topological_order() {
        let input = parse_input(EXAMPLE);
        topological_sort(&input);
    }
}
