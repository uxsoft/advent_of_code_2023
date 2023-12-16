use std::collections::BTreeSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn apply(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        match self {
            Direction::Right => Some((x + 1, y)),
            Direction::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Direction::Up => {
                if x > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Direction::Down => Some((x, y + 1)),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn energise(grid: &Vec<Vec<char>>, start: (i32, i32, Direction)) -> usize {
    let mut visited: BTreeSet<(i32, i32, Direction)> = BTreeSet::new();
    let mut energised: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut beams: Vec<(i32, i32, Direction)> = Vec::new();

    beams.push(start);

    while let Some((beam_x, beam_y, beam_dir)) = beams.pop() {
        if !visited.contains(&(beam_x, beam_y, beam_dir)) {
            visited.insert((beam_x, beam_y, beam_dir.clone()));
            energised.insert((beam_x, beam_y));
        } else {
            continue;
        }

        if let Some((next_x, next_y)) = beam_dir.apply(beam_x, beam_y) {
            if let Some(c) = grid
                .get(next_y as usize)
                .and_then(|row| row.get(next_x as usize))
            {
                match c {
                    '.' => {
                        beams.push((next_x, next_y, beam_dir));
                    }
                    '\\' if beam_dir == Direction::Right => {
                        beams.push((next_x, next_y, Direction::Down));
                    }
                    '\\' if beam_dir == Direction::Left => {
                        beams.push((next_x, next_y, Direction::Up));
                    }
                    '\\' if beam_dir == Direction::Up => {
                        beams.push((next_x, next_y, Direction::Left));
                    }
                    '\\' if beam_dir == Direction::Down => {
                        beams.push((next_x, next_y, Direction::Right));
                    }
                    '/' if beam_dir == Direction::Right => {
                        beams.push((next_x, next_y, Direction::Up));
                    }
                    '/' if beam_dir == Direction::Left => {
                        beams.push((next_x, next_y, Direction::Down));
                    }
                    '/' if beam_dir == Direction::Up => {
                        beams.push((next_x, next_y, Direction::Right));
                    }
                    '/' if beam_dir == Direction::Down => {
                        beams.push((next_x, next_y, Direction::Left));
                    }
                    '|' if beam_dir == Direction::Right || beam_dir == Direction::Left => {
                        beams.push((next_x, next_y, Direction::Up));
                        beams.push((next_x, next_y, Direction::Down));
                    }
                    '|' if beam_dir == Direction::Up || beam_dir == Direction::Down => {
                        beams.push((next_x, next_y, beam_dir));
                    }
                    '-' if beam_dir == Direction::Right || beam_dir == Direction::Left => {
                        beams.push((next_x, next_y, beam_dir));
                    }
                    '-' if beam_dir == Direction::Up || beam_dir == Direction::Down => {
                        beams.push((next_x, next_y, Direction::Left));
                        beams.push((next_x, next_y, Direction::Right));
                    }
                    _ => (),
                }
            }
        }
    }

    // println!("{energised:?}");
    return energised.len() - 1;
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);

    return energise(&grid, (-1, 0, Direction::Right));
}

pub fn part2(input: &str) -> usize {
    let grid = parse(input);

    let from_left = (0..grid.len()).map(|y| (-1_i32, y as i32, Direction::Right));
    let from_right = (0..grid.len()).map(|y| (grid[0].len() as i32, y as i32, Direction::Left));

    let from_top = (0..grid[0].len()).map(|x| (x as i32, -1_i32, Direction::Down));
    let from_bottom = (0..grid.len()).map(|x| (x as i32, grid.len() as i32, Direction::Up));

    let result = from_left.chain(from_right).chain(from_top).chain(from_bottom).map(|start| energise(&grid, start)).max().unwrap();

    return result;
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

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 46);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 7236);
    }

    #[test]
    fn part2_example() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 51);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
