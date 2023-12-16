use std::collections::BTreeSet;

use itertools::Itertools;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn apply(&self, x: usize, y: usize) -> Option<(usize, usize)> {
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

pub fn part1(input: &str) -> usize {
    let grid = parse(input);

    let mut energised: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut beams: Vec<(usize, usize, Direction)> = Vec::new();

    beams.push((0, 0, Direction::Right));

    while let Some((beam_x, beam_y, beam_dir)) = beams.pop() {
        if let Some((next_x, next_y)) = beam_dir.apply(beam_x, beam_y) {

            
        }
    }

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
        assert_eq!(result, 0);
    }

    // #[test]
    fn part2_example() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 0);
    }

    // #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
