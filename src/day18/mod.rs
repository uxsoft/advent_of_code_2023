use glam::IVec2;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            i => Err(format!("Unexpected direction input `{i}`")),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
    color: String,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, l, c) = s.split(" ").collect_tuple().unwrap();

        Ok(Instruction {
            direction: d.parse()?,
            length: l.parse().unwrap(),
            color: c.to_string(),
        })
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect_vec()
}

const GRID_WIDTH: usize = 400;
const GRID_HEIGHT: usize = 400;

fn print_grid(grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) -> String {
    grid.map(|line| {
        line.map(|i| match i {
            true => "#",
            false => " ",
        })
        .join("")
    })
    .join("\n")
}

pub fn part1(input: &str) -> usize {
    let instructions = parse(input);
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
    let trenches: Vec<IVec2> = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Right => {
                for _ in 0..instruction.length {
                    grid[y][x] = true;
                    trenches.push(IVec2::new(x, y));
                    x += 1;
                }
            }
            Direction::Left => {
                for _ in 0..instruction.length {
                    grid[y][x] = true;
                    if x > 0 {
                        x -= 1;
                    }
                }
            }
            Direction::Up => {
                for _ in 0..instruction.length {
                    grid[y][x] = true;
                    if y > 0 {
                        y -= 1;
                    }
                }
            }
            Direction::Down => {
                for _ in 0..instruction.length {
                    grid[y][x] = true;
                    y += 1;
                }
            }
        }
    }

    let s = print_grid(&grid);
    println!("{s}");

    return 1;
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

    const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    // #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
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
