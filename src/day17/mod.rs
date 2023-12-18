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
struct Crucible {
    pos: IVec2,
    dir: Direction,
    n_straight: u8,
}

impl Crucible {
    fn new(x: i32, y: i32, dir: Direction, n_straight: u8) -> Self {
        Self {
            pos: IVec2::new(x, y),
            dir,
            n_straight,
        }
    }
}

struct LargeCrucibleMechanics;
struct UltraCrucibleMechanics;

trait CrucibleMechanics {
    fn successors(&self, node: &Crucible) -> Vec<Crucible>;

    fn starts(&self) -> Vec<Crucible>;

    fn can_stop(&self, node: &Crucible) -> bool;
}

impl CrucibleMechanics for LargeCrucibleMechanics {
    fn successors(&self, node: &Crucible) -> Vec<Crucible> {
        use Direction::*;

        const MAX_N_STRAIGHT: u8 = 3;

        let mut result: Vec<Crucible> = Vec::new();

        // Try add Down
        if node.dir != Up && !(node.dir == Down && node.n_straight >= MAX_N_STRAIGHT) {
            let m_straight = if node.dir == Down {
                node.n_straight + 1
            } else {
                1
            };
            result.push(Crucible::new(node.pos.x, node.pos.y + 1, Down, m_straight));
        }

        // Try add Up
        if node.dir != Down
            && node.pos.y > 0
            && !(node.dir == Up && node.n_straight >= MAX_N_STRAIGHT)
        {
            let m_straight = if node.dir == Up {
                node.n_straight + 1
            } else {
                1
            };
            result.push(Crucible::new(node.pos.x, node.pos.y - 1, Up, m_straight));
        }

        // Try add Right
        if node.dir != Left && !(node.dir == Right && node.n_straight >= MAX_N_STRAIGHT) {
            let m_straight = if node.dir == Right {
                node.n_straight + 1
            } else {
                1
            };
            result.push(Crucible::new(node.pos.x + 1, node.pos.y, Right, m_straight));
        }

        // Try add Left
        if node.dir != Right
            && node.pos.x > 0
            && !(node.dir == Left && node.n_straight >= MAX_N_STRAIGHT)
        {
            let m_straight = if node.dir == Left {
                node.n_straight + 1
            } else {
                1
            };
            result.push(Crucible::new(node.pos.x - 1, node.pos.y, Left, m_straight));
        }

        return result;
    }

    fn starts(&self) -> Vec<Crucible> {
        vec![Crucible::new(0, 0, Direction::Right, 0)]
    }

    fn can_stop(&self, node: &Crucible) -> bool {
        true
    }
}

impl CrucibleMechanics for UltraCrucibleMechanics {
    fn successors(&self, node: &Crucible) -> Vec<Crucible> {
        use Direction::*;
        const MAX_N_STRAIGHT: u8 = 10;
        const MIN_N_STRAIGHT: u8 = 4;

        let mut result: Vec<Crucible> = Vec::new();

        if node.n_straight < MIN_N_STRAIGHT {
            result.push(match node.dir {
                Up => Crucible::new(node.pos.x, node.pos.y - 1, Up, node.n_straight + 1),
                Down => Crucible::new(node.pos.x, node.pos.y + 1, Down, node.n_straight + 1),
                Right => Crucible::new(node.pos.x + 1, node.pos.y, Right, node.n_straight + 1),
                Left => Crucible::new(node.pos.x - 1, node.pos.y, Left, node.n_straight + 1),
            })
        } else {
            // Try add Down
            if node.dir != Up && !(node.dir == Down && node.n_straight >= MAX_N_STRAIGHT) {
                let m_straight = if node.dir == Down {
                    node.n_straight + 1
                } else {
                    1
                };
                result.push(Crucible::new(node.pos.x, node.pos.y + 1, Down, m_straight));
            }

            // Try add Up
            if node.dir != Down
                && node.pos.y > 0
                && !(node.dir == Up && node.n_straight >= MAX_N_STRAIGHT)
            {
                let m_straight = if node.dir == Up {
                    node.n_straight + 1
                } else {
                    1
                };
                result.push(Crucible::new(node.pos.x, node.pos.y - 1, Up, m_straight));
            }

            // Try add Right
            if node.dir != Left && !(node.dir == Right && node.n_straight >= MAX_N_STRAIGHT) {
                let m_straight = if node.dir == Right {
                    node.n_straight + 1
                } else {
                    1
                };
                result.push(Crucible::new(node.pos.x + 1, node.pos.y, Right, m_straight));
            }

            // Try add Left
            if node.dir != Right
                && node.pos.x > 0
                && !(node.dir == Left && node.n_straight >= MAX_N_STRAIGHT)
            {
                let m_straight = if node.dir == Left {
                    node.n_straight + 1
                } else {
                    1
                };
                result.push(Crucible::new(node.pos.x - 1, node.pos.y, Left, m_straight));
            }
        }

        return result;
    }

    fn starts(&self) -> Vec<Crucible> {
        vec![
            Crucible::new(0, 0, Direction::Right, 0),
            Crucible::new(0, 0, Direction::Down, 0),
        ]
    }

    fn can_stop(&self, node: &Crucible) -> bool {
        node.n_straight >= 4
    }
}

fn find_shortest_path(
    grid: &HashMap<IVec2, u32>,
    start: Crucible,
    mechanics: &impl CrucibleMechanics,
) -> (Vec<Crucible>, u32) {
    let max_x = grid.keys().map(|i| i.x).max().unwrap();
    let max_y = grid.keys().map(|i| i.y).max().unwrap();

    let goal_pos = IVec2::new(max_x, max_y);
    let (path, distance) = dijkstra(
        &start,
        |node| {
            mechanics
                .successors(node)
                .iter()
                .filter_map(|i| grid.get(&i.pos).map(|cost| (*i, *cost)))
                .collect::<Vec<(Crucible, u32)>>()
        },
        |node| node.pos == goal_pos && mechanics.can_stop(node),
    )
    .expect("should have a valid path");

    return (path, distance);
}

fn print_path(path: &Vec<Crucible>) -> String {
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

    let mechanics = LargeCrucibleMechanics {};

    let (p, d) = &mechanics
        .starts()
        .iter()
        .map(|start| find_shortest_path(&grid, *start, &mechanics))
        .min_by_key(|(_, d)| *d)
        .unwrap();

    // println!("{}", print_path(p));

    return *d;
}

pub fn part2(input: &str) -> u32 {
    let grid = parse(input);

    let mechanics = UltraCrucibleMechanics {};

    let (p, d) = &mechanics
        .starts()
        .iter()
        .map(|start| find_shortest_path(&grid, *start, &mechanics))
        .min_by_key(|(_, d)| *d)
        .unwrap();

    // println!("{}", print_path(p));

    return *d;
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

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 94);
    }

    #[test]
    fn part2_example2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        let result = part2(input);
        assert_eq!(result, 71);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 1017);
    }
}
