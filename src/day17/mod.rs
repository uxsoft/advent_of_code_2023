use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<IVec2, u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Node {
    pos: IVec2,
    dir: Direction,
    n_straight: u8,
}

impl Node {
    fn new(x: i32, y: i32, dir: Direction, n_straight: u8) -> Self {
        Self {
            pos: IVec2::new(x, y),
            dir,
            n_straight,
        }
    }

    fn successors(&self) -> Vec<Node> {
        use Direction::*;

        let mut result: Vec<Node> = Vec::new();

        // Try add Down
        if self.dir != Up && !(self.dir == Down && self.n_straight >= 3) {
            let m_straight = if self.dir == Down {
                self.n_straight + 1
            } else {
                1
            };
            result.push(Node::new(self.pos.x, self.pos.y + 1, Down, m_straight));
        }

        // Try add Up
        if self.dir != Down && self.pos.y > 0 && !(self.dir == Up && self.n_straight >= 3) {
            let m_straight = if self.dir == Up {
                self.n_straight + 1
            } else {
                1
            };
            result.push(Node::new(self.pos.x, self.pos.y - 1, Up, m_straight));
        }

        // Try add Right
        if self.dir != Left && !(self.dir == Right && self.n_straight >= 3) {
            let m_straight = if self.dir == Right {
                self.n_straight + 1
            } else {
                1
            };
            result.push(Node::new(self.pos.x + 1, self.pos.y, Right, m_straight));
        }

        // Try add Left
        if self.dir != Right && self.pos.x > 0 && !(self.dir == Left && self.n_straight >= 3) {
            let m_straight = if self.dir == Left {
                self.n_straight + 1
            } else {
                1
            };
            result.push(Node::new(self.pos.x - 1, self.pos.y, Left, m_straight));
        }

        return result;
    }
}

fn find_shortest_path(grid: &HashMap<IVec2, u32>) -> (Vec<Node>, u32) {
    let max_x = grid.keys().map(|i| i.x).max().unwrap();
    let max_y = grid.keys().map(|i| i.y).max().unwrap();

    let start = Node::new(0, 0, Direction::Right, 0);
    let goal_pos = IVec2::new(max_x, max_y);
    let (path, distance) = dijkstra(
        &start,
        |node| {
            node.successors()
                .iter()
                .filter_map(|i| grid.get(&i.pos).map(|cost| (*i, *cost)))
                .collect::<Vec<(Node, u32)>>()
        },
        |node| node.pos == goal_pos,
    )
    .expect("should have a valid path");

    return (path, distance);
}

fn print_path(path: &Vec<Node>) -> String {
    let max_x = path.iter().map(|i| i.pos.x).max().unwrap();
    let max_y = path.iter().map(|i| i.pos.y).max().unwrap();

    (0..=max_y)
        .map(|y| {
            (0..=max_x)
                .map(|x| {
                    let on_path = path.iter().any(|n| n.pos.x == x && n.pos.y == y);
                    if on_path {
                        '#'
                    } else {
                        ' '
                    }
                })
                .join("")
        })
        .join("\n")
}

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);

    let (p, d) = find_shortest_path(&grid);

    println!("{}", print_path(&p));

    return d;
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

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 102);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 843);
    }

    // #[test]
    fn part2_example() {
        let input = "";
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
