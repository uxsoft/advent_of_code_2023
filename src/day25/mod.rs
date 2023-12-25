use indicatif::ProgressIterator;
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone)]
struct Graph<'a> {
    nodes: BTreeMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    fn parse(input: &str) -> Graph {
        let edges = input
            .lines()
            .map(|line| {
                let (name, edges_str) = line.split_once(": ").unwrap();

                let vertices = edges_str.split(' ').collect();

                (name, vertices)
            })
            .collect();

        Graph { nodes: edges }
    }

    fn edges(&self) -> Vec<(&'a str, &'a str)> {
        self.nodes
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(|to| (*from, *to)))
            .collect_vec()
    }

    fn fix_edges(&mut self) {
        let edges = self.edges();

        for (from, to) in edges {
            self.nodes
                .entry(to)
                .and_modify(|froms| {
                    froms.insert(from);
                })
                .or_insert(HashSet::from([from]));
        }
    }

    fn _to_gephi_nodes(&self) -> String {
        self.nodes.keys().join("\n")
    }

    fn _to_gephi_edges(&self) -> String {
        self.edges()
            .into_iter()
            .map(|(from, to)| format!("{from};{to};Undirected"))
            .join("\n")
    }

    fn remove_edge(&mut self, from: &'a str, to: &'a str) {
        self.nodes.get_mut(from).unwrap().remove(to);
        self.nodes.get_mut(to).unwrap().remove(from);
    }

    fn insert_edge(&mut self, from: &'a str, to: &'a str) {
        self.nodes.get_mut(from).unwrap().insert(to);
        self.nodes.get_mut(to).unwrap().insert(from);
    }

    fn strongly_connected_component(&self, start: &'a str) -> HashSet<&'a str> {
        let mut visited = HashSet::new();
        let mut queue = vec![];
        queue.push(start);

        while let Some(node) = queue.pop() {
            if !visited.contains(node) {
                visited.insert(node);

                for edge in self
                    .nodes
                    .get(node)
                    .expect(format!("Expected edges to contain {node}").as_str())
                {
                    queue.push(edge);
                }
            }
        }

        return visited;
    }

    fn shortest_path(&self, start: &'a str, end: &'a str) -> Vec<&str> {
        let (path, _) = pathfinding::directed::dijkstra::dijkstra(
            &start,
            |n| self.nodes.get(n).unwrap().iter().map(|next| (*next, 1)),
            |n| n == &end,
        )
        .unwrap();
        return path;
    }

    fn min_cut_candidates(&self, n: usize) -> Vec<&str> {
        let mut counts = BTreeMap::new();

        for (from, to) in self
            .nodes
            .keys()
            .tuple_combinations()
            .take(n)
            // .progress_count(n as u64)
        {
            let path = self.shortest_path(from, to);
            for node in path {
                counts
                    .entry(node)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        return counts
            .iter()
            .sorted_by_key(|(_, count)| -*count)
            .take(6)
            .map(|(node, _)| *node)
            .collect();
    }
}

pub fn part1(input: &str) -> usize {
    let mut graph = Graph::parse(input);
    graph.fix_edges();

    let max_n = graph.nodes.len() * (graph.nodes.len() - 1);
    let candidates = graph.min_cut_candidates(200.max(max_n / 100));

    // dbg!(&candidates);

    let mut partitioned_graph = graph.clone();
    for (from, to) in candidates.iter().tuple_combinations() {
        partitioned_graph.remove_edge(*from, *to);
    }

    let sizes = candidates
        .iter()
        .map(|node| partitioned_graph.strongly_connected_component(node).len())
        .sorted()
        .dedup()
        .collect_vec();

    // dbg!(&sizes);

    return sizes.iter().product();
}

pub fn process(input: String) {
    use std::time::Instant;
    let now = Instant::now();
    let result = part1(&input);
    println!("Result: {result}");
    println!("Finished in: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        // cnr, hcd, bqp, fqr, fhv, zsp
        assert_eq!(result, 54);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 568214);
    }
}
