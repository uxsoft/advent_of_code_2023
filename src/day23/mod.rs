use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Pos {
    Trail,
    Forest,
    SlopeRight,
    SlopeDown,
}

impl Pos {
    fn parse(input: char) -> Pos {
        use Pos::*;

        match input {
            '#' => Forest,
            '.' => Trail,
            '>' => SlopeRight,
            'v' => SlopeDown,
            other => unreachable!("Encountered an unexpected grid char `{other}`"),
        }
    }

    fn is_walkable(&self) -> bool {
        match self {
            Pos::Forest => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
struct HikingMap {
    grid: Vec<Vec<Pos>>,
}

impl HikingMap {
    fn parse(input: &str) -> HikingMap {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Pos::parse).collect())
            .collect();

        HikingMap { grid }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Pos> {
        self.grid.get(y).and_then(|row| row.get(x))
    }

    fn successors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let options = match self.grid.get(y).unwrap().get(x).unwrap() {
            Pos::Trail => vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)],
            Pos::Forest => vec![],
            Pos::SlopeRight => vec![(x + 1, y)],
            Pos::SlopeDown => vec![(x, y + 1)],
        };

        options
            .into_iter()
            .filter(|(x, y)| self.get(*x, *y).unwrap_or(&Pos::Forest).is_walkable())
            .collect()
    }

    fn successors_no_slopes(
        &self,
        pos: (usize, usize),
        visited: &BTreeSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let options = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        options
            .into_iter()
            .filter(|(x, y)| self.get(*x, *y).unwrap_or(&Pos::Forest).is_walkable())
            .filter(|p| !visited.contains(p))
            .collect()
    }

    fn successors_no_slopes_skip(&self, start: (usize, usize)) -> Vec<((usize, usize), usize)> {
        self.successors_no_slopes(start, &BTreeSet::new())
            .into_iter()
            .map(|first_step| {
                let mut visited: BTreeSet<_> = vec![start, first_step].into_iter().collect();
                let mut distance = 0;
                let mut node = first_step;

                loop {
                    let nexts = self.successors_no_slopes(node, &visited);
                    if nexts.len() == 1 {
                        node = nexts[0];
                        distance += 1;
                        visited.insert(node);
                    } else {
                        return (node, distance + 1);
                    }
                }
            })
            .collect()
    }

    fn longest_path(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        visited: HashSet<(usize, usize)>,
    ) -> usize {
        if start == end {
            0
        } else {
            self.successors(start)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .map(|next| {
                    let mut new_visited = visited.clone();
                    new_visited.insert(next);
                    let longest = self.longest_path(next, end, new_visited) + 1;
                    longest
                })
                .max()
                .unwrap_or(0)
        }
    }

    fn print_path(&self, path: &BTreeSet<(usize, usize)>) -> String {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, p)| match p {
                        _ if path.contains(&(x, y)) => "O",
                        Pos::Forest => "#",
                        Pos::Trail => ".",
                        _ => "_",
                    })
                    .join("")
            })
            .join("\n")
    }
}

struct HikingGraph {
    distances: BTreeMap<(usize, usize), BTreeMap<(usize, usize), usize>>,
    neighbors: BTreeMap<(usize, usize), BTreeSet<(usize, usize)>>,
}

impl HikingGraph {
    fn new(map: &HikingMap, start: (usize, usize)) -> HikingGraph {
        let mut distances: BTreeMap<(usize, usize), BTreeMap<(usize, usize), usize>> =
            BTreeMap::new();
        let mut neighbors: BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> = BTreeMap::new();

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if !visited.contains(&node) {
                visited.insert(node);

                let distances_of_node = distances.entry(node).or_default();
                let neighbors_of_node = neighbors.entry(node).or_default();
                for (next, distance) in map.successors_no_slopes_skip(node) {
                    distances_of_node.insert(next, distance);
                    neighbors_of_node.insert(next);
                    queue.push_back(next);
                }
            }
        }

        HikingGraph {
            distances,
            neighbors,
        }
    }

    fn longest_path(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        visited: BTreeSet<(usize, usize)>,
    ) -> Option<(BTreeSet<(usize, usize)>, usize)> {
        if start == end {
            let mut new_visited = visited.clone();
            new_visited.insert(end);
            return Some((new_visited, 0));
        } else {
            let successors = self.neighbors.get(&start).unwrap();

            return successors
                .into_iter()
                .filter(|n| !visited.contains(n))
                .filter_map(|next| {
                    let mut new_visited = visited.clone();
                    new_visited.insert(*next);

                    let distance = self.distances.get(&start).unwrap().get(&next).unwrap();
                    self.longest_path(*next, end, new_visited)
                        .map(|(p, d)| (p, d + distance))
                })
                .max_by_key(|(_, d)| *d);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let map = HikingMap::parse(input);

    let starts = map
        .grid
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|(x, p)| *p == &Pos::Trail)
        .map(|(x, p)| (x, 0))
        .collect_vec();

    let end = (map.grid.last().unwrap().len() - 2, map.grid.len() - 1);

    let result = starts
        .iter()
        .map(|start| map.longest_path(*start, end, HashSet::new()))
        .max()
        .unwrap();

    return result;
}

pub fn part2(input: &str) -> usize {
    let map = HikingMap::parse(input);

    let starts = map
        .grid
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|(x, p)| *p == &Pos::Trail)
        .map(|(x, p)| (x, 0))
        .collect_vec();

    let end = (map.grid.last().unwrap().len() - 2, map.grid.len() - 1);

    let graph = HikingGraph::new(&map, starts[0]);
    let nodes = graph.neighbors.keys().collect_vec();
    let (_path, distance) = graph.longest_path(starts[0], end, BTreeSet::new()).unwrap();
    println!("End: {:?}, nodes: {:?}", end, nodes);

    return distance;
}

pub fn process(input: String) {
    use std::time::Instant;
    let now = Instant::now();
    let result = part2(&input);
    println!("Result: {result}");
    println!("Finished in: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 94);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 1930);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 154);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 6230);
    }
}
