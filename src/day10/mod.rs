use itertools::Itertools;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    pub fn translate(&self, point: &(usize, usize)) -> (usize, usize) {
        use Direction::*;

        match self {
            North => (point.0, point.1 - 1),
            South => (point.0, point.1 + 1),
            East => (point.0 + 1, point.1),
            West => (point.0 - 1, point.1),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    pub fn parse(input: char) -> Pipe {
        use Pipe::*;

        match input {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            p => panic!("Found an unexpected pipe shape {p}"),
        }
    }

    fn inlets(&self) -> Vec<Direction> {
        use Direction::*;
        use Pipe::*;
        match self {
            NorthSouth => vec![North, South],
            EastWest => vec![East, West],
            NorthEast => vec![North, East],
            NorthWest => vec![North, West],
            SouthWest => vec![South, West],
            SouthEast => vec![South, East],
            Ground => vec![],
            Start => vec![North, South, East, West],
        }
    }

    fn to_char(&self) -> char {
        match self {
            Pipe::NorthSouth => '║',
            Pipe::EastWest => '═',
            Pipe::NorthEast => '╚',
            Pipe::NorthWest => '╝',
            Pipe::SouthWest => '╗',
            Pipe::SouthEast => '╔',
            Pipe::Ground => ' ',
            Pipe::Start => '╬',
        }
    }

    fn traverse(&self, direction: Direction) -> Direction {
        use Direction::*;
        use Pipe::*;

        match (self, direction) {
            (NorthSouth, North) => North,
            (NorthSouth, South) => South,
            (EastWest, East) => East,
            (EastWest, West) => West,
            (NorthEast, West) => North,
            (NorthEast, South) => East,
            (NorthWest, South) => West,
            (NorthWest, East) => North,
            (SouthWest, North) => West,
            (SouthWest, East) => South,
            (SouthEast, North) => East,
            (SouthEast, West) => South,
            (p, d) => panic!("Didn't expect to traverse pipe ({p:?} from {d:?}"),
        }
    }
}

#[derive(Debug)]
struct PipeMaze {
    pipes: Vec<Vec<Pipe>>,
}

impl PipeMaze {
    fn parse(input: &str) -> PipeMaze {
        let pipes = input
            .lines()
            .map(|line| line.chars().map(Pipe::parse).collect_vec())
            .collect_vec();

        PipeMaze { pipes }
    }

    fn get_at(&self, coords: (usize, usize)) -> &Pipe {
        self.pipes
            .get(coords.1)
            .and_then(|row| row.get(coords.0))
            .unwrap_or(&Pipe::Ground)
    }

    fn start(&self) -> (usize, usize) {
        self.pipes
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, pipe)| match pipe {
                    Pipe::Start => Some((x, y)),
                    _ => None,
                })
            })
            .unwrap()
    }

    fn walk_circuit(&self) -> HashSet<(usize, usize)> {
        use Direction::*;

        let start = self.start();
        let mut path = HashSet::new();

        let (mut node, mut node_direction) = vec![North, East, South, West]
            .iter()
            .find_map(|dir| {
                let next_node = dir.translate(&start);
                let next_pipe = self.get_at(next_node);

                if next_pipe.inlets().contains(&dir.reverse()) {
                    Some((next_node, *dir))
                } else {
                    None
                }
            })
            .unwrap();

        path.insert(node);

        while node != start {
            let pipe = self.get_at(node);

            node_direction = pipe.traverse(node_direction);
            node = node_direction.translate(&node);

            path.insert(node);
        }

        return path;
    }

    fn points_inside(&self, path: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        use Pipe::*;
        
        self.pipes
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, _)| {
                    if !path.contains(&(x, y)) {
                        let crossed_pipes = (0..x)
                            .filter(|x| path.contains(&(*x, y)))
                            .filter(|x| match self.get_at((*x, y)) {
                                NorthSouth | NorthWest | NorthEast | Start => true,
                                _ => false,
                            })
                            .count();

                        if crossed_pipes % 2 == 1 {
                            return Some((x, y));
                        }
                    }
                    return None;
                })
            })
            .collect::<HashSet<_>>()
    }

    fn to_string(
        &self,
        path: &HashSet<(usize, usize)>,
        highlights: &HashSet<(usize, usize)>,
    ) -> String {
        let hash_path = path.iter().collect::<HashSet<_>>();

        self.pipes
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, pipe)| {
                        if hash_path.contains(&(x, y)) {
                            pipe.to_char()
                        } else if highlights.contains(&(x, y)) {
                            '.'
                        } else {
                            ' '
                        }
                    })
                    .join("")
            })
            .join("\n")
    }
}

pub fn part1(input: &str) -> usize {
    let maze = PipeMaze::parse(&input);

    let path = maze.walk_circuit();

    return path.len() / 2;
}

fn part2(input: &str) -> usize {
    let maze = PipeMaze::parse(input);
    let path = maze.walk_circuit();
    let highlights = maze.points_inside(&path);

    let display = maze.to_string(&path, &highlights);
    println!("{}", display);

    return highlights.len();
}

pub fn process(input: String) {
    let result = part2(&input);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_test() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 7086);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 317);
    }

    #[test]
    fn part2_example1() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let result = part2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_example2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = part2(input);
        assert_eq!(result, 8);
    }
}
