use std::collections::{BTreeMap, VecDeque};

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

    fn longest_path2(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let mut distances = BTreeMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((start, 1));

        while let Some((pos, d)) = queue.pop_front() {
            let distance = *distances.get(&pos).unwrap_or(&0);
            if d > distance {
                distances.insert(pos, d);

                for next in self.successors(pos) {
                    queue.push_back((next, d + 1));
                }
            }

            println!("{pos:?}: {d}");
        }

        return *distances.get(&end).unwrap();
    }

    fn longest_path(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        if start == end {
            0
        } else {
            self.successors(start)
                .into_iter()
                .map(|next| self.longest_path(next, end))
                .max()
                .unwrap()
                + 1
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

    let end = (map.grid.len() - 1, map.grid.last().unwrap().len() - 2);

    let result = starts
        .iter()
        .map(|start| map.longest_path(*start, end))
        .max();

    return 0;
}

pub fn part2(input: &str) -> usize {
    return 0;
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

    // #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 0);
    }

    // #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    // #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
