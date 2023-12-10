use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

#[derive(PartialEq, Debug)]
enum Inlet {
    North,
    South,
    East,
    West,
}

impl Inlet {
    pub fn reverse(&self) -> Inlet {
        use Inlet::*;
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

    fn inlets(&self) -> Vec<Inlet> {
        use Inlet::*;
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
                        Inlet::North => (x as i32, y as i32 - 1),
                        Inlet::South => (x as i32, y as i32 + 1),
                        Inlet::East => (x as i32 + 1, y as i32),
                        Inlet::West => (x as i32 - 1, y as i32),
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

    // fn distances(&self) -> {
    //         // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
    //         // for each edge (u, v) do
    //         //     dist[u][v] ← w(u, v)  // The weight of the edge (u, v)
    //         // for each vertex v do
    //         //     dist[v][v] ← 0
    //         // for k from 1 to |V|
    //         //     for i from 1 to |V|
    //         //         for j from 1 to |V|
    //         //             if dist[i][j] > dist[i][k] + dist[k][j]
    //         //                 dist[i][j] ← dist[i][k] + dist[k][j]
    //         //             end if
    // }
}

pub fn process(input: String) {
    let maze = PipeMaze::parse(&input);
    let start = maze.start();
    let connections = maze.connections();

    println!("{maze:?}");
    println!("Starting at {start:?}");
    println!("{connections:?}");

    for start_plus in connections.get(&start).unwrap() {
        let mut visited = HashSet::new();
        let mut current = start_plus;
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

}
