use itertools::Itertools;
use std::collections::{BTreeMap, HashSet, VecDeque, HashMap};
use crate::day10::Pipe::{EastWest, NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest};

#[derive(PartialEq, Debug)]
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

    fn to_directional_char(&self, direction: &Direction) -> char {
        use Pipe::*;
        use Direction::*;

        match (self, direction) {
            (NorthSouth, North) => '↑',
            (NorthSouth, South) => '↓',
            (EastWest, East) => '→',
            (EastWest, West) => '←',
            (NorthEast, North) => '⬑',
            (NorthEast, East) => '↳',
            (NorthWest, North) => '⬏',
            (NorthWest, West) => '↲',
            (SouthWest, South) => '⬎',
            (SouthWest, West) => '↰',
            (SouthEast, South) => '⬐',
            (SouthEast, East) => '↱',
            (p, _) => p.to_char()
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

    fn get(&self, x: i32, y: i32) -> Pipe {
        if x >= 0 && y >= 0 {
            *self
                .pipes
                .get(y as usize)
                .and_then(|row| row.get(x as usize))
                .unwrap_or(&Pipe::Ground)
        } else {
            Pipe::Ground
        }
    }

    fn connections(&self) -> BTreeMap<(usize, usize), Vec<(usize, usize)>> {
        let mut endings: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();

        for (y, row) in self.pipes.iter().enumerate() {
            for (x, pipe) in row.iter().enumerate() {
                for inlet in pipe.inlets() {
                    let neighbor_coords = match inlet {
                        Direction::North => (x as i32, y as i32 - 1),
                        Direction::South => (x as i32, y as i32 + 1),
                        Direction::East => (x as i32 + 1, y as i32),
                        Direction::West => (x as i32 - 1, y as i32),
                    };

                    if self
                        .get(neighbor_coords.0, neighbor_coords.1)
                        .inlets()
                        .contains(&inlet.reverse())
                    {
                        let v = endings.entry((x, y)).or_default();
                        v.push((neighbor_coords.0 as usize, neighbor_coords.1 as usize));
                    }
                }
            }
        }

        return endings;
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

    fn find_loop(&self, min_length: usize) -> Vec<(usize, usize)> {
        let start = self.start();
        let connections = self.connections();

        connections
            .get(&start)
            .unwrap()
            .iter()
            .find_map(|start_next| {
                let mut visited: Vec<(usize, usize)> = Vec::new();
                let mut current = *start_next;

                while current != start {
                    visited.push(current);
                    let next = connections
                        .get(&current)
                        .unwrap()
                        .iter()
                        .find(|n| !visited.contains(n));

                    if let Some(next) = next {
                        current = *next;
                    } else {
                        return None; // Not a loop
                    }
                }
                visited.push(start);

                return if visited.len() > min_length {
                    Some(visited)
                } else {
                    None
                };
            })
            .unwrap()
    }

    fn to_string(
        &self,
        path: &Vec<(usize, usize)>,
        highlights: &HashSet<(usize, usize)>,
    ) -> String {
        self.pipes
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, pipe)| {
                        if path.contains(&(x, y)) {
                            pipe.to_char()
                        } else if highlights.contains(&(x, y)) {
                            '*'
                        } else {
                            '.'
                        }
                    })
                    .join("")
            })
            .join("\n")
    }
}

pub fn part1(input: String) {
    let maze = PipeMaze::parse(&input);
    let start = maze.start();
    let connections = maze.connections();

    let mut visited = HashSet::new();
    let mut current = connections.get(&start).unwrap().get(0).unwrap();
    let mut length = 1;

    while current != &start {
        println!("Visiting: {current:?}");

        visited.insert(current);
        let next = connections
            .get(&current)
            .unwrap()
            .iter()
            .find(|n| !visited.contains(n));

        if let Some(next) = next {
            length += 1;
            current = next;
        } else {
            break;
        }
    }

    println!("Loop length: {length}");
}

fn part2(input: String) {
    let maze = PipeMaze::parse(&input);
    let path = maze.find_loop(4);
    let mut highlights: HashSet<(usize, usize)> = HashSet::new();
    let mut directions: HashMap<(usize, usize), Direction> = HashMap::new();


    // for ((cx, cy), (nx, ny)) in path.iter().tuple_windows() {
    //     let pipe = maze.get(*cx as i32, *cy as i32);
    //
    //     if cx == nx && ny > cy {
    //         //North ^
    //         directions.insert((*cx, *cy), Direction::North);
    //         highlights.insert((cx - 1, *cy));
    //         if pipe == Pipe::NorthEast {}
    //     } else if cx == nx && cy > ny {
    //         // South
    //         directions.insert((*cx, *cy), Direction::South);
    //         highlights.insert((cx + 1, *cy));
    //     } else if cx > nx && cy == ny {
    //         // West <-
    //         directions.insert((*cx, *cy), Direction::West);
    //         highlights.insert((*cx, cy + 1));
    //         if pipe == Pipe::NorthEast {}
    //     } else if nx > cx && cy == ny {
    //         // East ->
    //         directions.insert((*cx, *cy), Direction::East);
    //         highlights.insert((*cx, cy - 1));
    //     } else {
    //         println!("Soft panic: unexpected {cx},{cy} to {nx},{ny}");
    //     }
    // }

    // println!("Path: {:?}", path);

    for (y, line) in maze.pipes.iter().enumerate() {
        for (x, pipe) in line.iter().enumerate() {
            if !path.contains(&(x, y)) {
                let crossed_pipes = (0..x)
                    .filter(|x| path.contains(&(*x, y)))
                    .filter(|x| match maze.get(*x as i32, y as i32) {
                        Pipe::NorthSouth => true,
                        Pipe::NorthWest => true,
                        Pipe::NorthEast => true,
                        Pipe::Start => true,
                        _ => false
                    })
                    .count();

                if crossed_pipes % 2 == 1 {
                    highlights.insert((x, y));
                }
            }
        }
    }

    let display = maze.pipes
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, pipe)| {
                    if path.contains(&(x, y)) {
                        if directions.contains_key(&(x, y)) {
                            pipe.to_directional_char(directions.get(&(x, y)).unwrap())
                        } else {
                            pipe.to_char()
                        }
                    } else if highlights.contains(&(x, y)) {
                        '.'
                    } else {
                        ' '
                    }
                })
                .join("")
        })
        .join("\n");

    println!("{display}");
    println!("Result: {}", highlights.len());
}

pub fn process(input: String) {
    part2(input);
}
