use glam::I64Vec2;
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

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

impl Instruction {
    fn to_correct(&self) -> Instruction {
        let length = &self.color[2..7];
        let direction = &self.color[7..8];

        Instruction {
            direction: match direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!("Didn't expect this direction"),
            },
            length: usize::from_str_radix(length, 16).unwrap(),
            color: "".to_string(),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect_vec()
}

fn print_trenches(trenches: &HashSet<I64Vec2>, detections: &HashSet<I64Vec2>) -> String {
    let max_x = trenches.iter().map(|i| i.x).max().unwrap();
    let max_y = trenches.iter().map(|i| i.y).max().unwrap();
    let min_x = trenches.iter().map(|i| i.x).min().unwrap();
    let min_y = trenches.iter().map(|i| i.y).min().unwrap();

    (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| {
                    let key = I64Vec2::new(x, y);
                    match trenches.contains(&key) {
                        true => "#",
                        false if detections.contains(&key) => ".",
                        false => " ",
                    }
                })
                .join("")
        })
        .join("\n")
}

fn trenches(instructions: &Vec<Instruction>) -> HashSet<I64Vec2> {
    let mut trenches: HashSet<I64Vec2> = HashSet::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Right => {
                for _ in 0..instruction.length {
                    trenches.insert(I64Vec2::new(x, y));
                    x += 1;
                }
            }
            Direction::Left => {
                for _ in 0..instruction.length {
                    trenches.insert(I64Vec2::new(x, y));
                    x -= 1;
                }
            }
            Direction::Up => {
                for _ in 0..instruction.length {
                    trenches.insert(I64Vec2::new(x, y));
                    y -= 1;
                }
            }
            Direction::Down => {
                for _ in 0..instruction.length {
                    trenches.insert(I64Vec2::new(x, y));
                    y += 1;
                }
            }
        }
    }

    return trenches;
}

fn vertices(instructions: &Vec<Instruction>) -> Vec<I64Vec2> {
    let mut vertices: Vec<I64Vec2> = Vec::new();
    vertices.push(I64Vec2::new(0, 0));

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Right => {
                x += instruction.length as i64;
                vertices.push(I64Vec2::new(x, y));
            }
            Direction::Left => {
                x -= instruction.length as i64;
                vertices.push(I64Vec2::new(x, y));
            }
            Direction::Up => {
                y -= instruction.length as i64;
                vertices.push(I64Vec2::new(x, y));
            }
            Direction::Down => {
                y += instruction.length as i64;
                vertices.push(I64Vec2::new(x, y));
            }
        }
    }

    return vertices;
}

fn flood_fill(start: I64Vec2, trenches: &HashSet<I64Vec2>) -> HashSet<I64Vec2> {
    let mut queue: Vec<I64Vec2> = Vec::new();
    queue.push(start);

    let mut visited: HashSet<I64Vec2> = HashSet::new();

    while let Some(n) = queue.pop() {
        if !trenches.contains(&n) && !visited.contains(&n) {
            visited.insert(n);

            queue.push(I64Vec2::new(n.x, n.y + 1));
            queue.push(I64Vec2::new(n.x, n.y - 1));
            queue.push(I64Vec2::new(n.x + 1, n.y));
            queue.push(I64Vec2::new(n.x - 1, n.y));
        }
    }

    return visited;
}

fn polygon_area(vertices: &Vec<I64Vec2>) -> f64 {
    let mut total = 0.;

    for i in 0..vertices.len() {
        let add_x = vertices[i].x;
        let add_y = vertices[if i == vertices.len() - 1 { 0 } else { i + 1 }].y;
        let sub_x = vertices[if i == vertices.len() - 1 { 0 } else { i + 1 }].x;
        let sub_y = vertices[i].y;

        total += add_x as f64 * add_y as f64 * 0.5;
        total -= sub_x as f64 * sub_y as f64 * 0.5;
    }

    return total.abs();
}

pub fn part1(input: &str) -> usize {
    let instructions = parse(input);

    let trenches = trenches(&instructions);

    let fill = flood_fill(I64Vec2::new(1, 1), &trenches);

    return trenches.len() + fill.len();
}

pub fn part2(input: &str) -> usize {
    let instructions = parse(input).iter().map(|i| i.to_correct()).collect_vec();
    let vertices = vertices(&instructions);
    
    let area = polygon_area(&vertices);

    let perimeter_length: usize = instructions.iter().map(|i| i.length).sum();

    let total_area = area as f64 + (perimeter_length as f64 / 2.) + 1.;

    return total_area as usize;
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

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 62);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 49578);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 52885384955882);
    }
}
