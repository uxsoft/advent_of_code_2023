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
    fn apply(&self, x: usize, y: usize) -> (usize, usize, Direction) {
        match self {
            Direction::Right => (x + 1, y, *self),
            Direction::Left => (x - 1, y, *self),
            Direction::Up => (x, y - 1, *self),
            Direction::Down => (x, y + 1, *self),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn energise(grid: &Vec<Vec<char>>, start: (usize, usize, Direction)) -> usize {
    use Direction::*;

    let mut visited: BTreeSet<(usize, usize, Direction)> = BTreeSet::new();
    let mut energised: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut beams: Vec<(usize, usize, Direction)> = Vec::new();

    beams.push(start);

    while let Some((x, y, d)) = beams.pop() {
        if !visited.contains(&(x, y, d)) {
            visited.insert((x, y, d));
            energised.insert((x, y));
        } else {
            continue;
        }

        if let Some(c) = grid.get(x as usize).and_then(|row| row.get(y as usize)) {
            match c {
                '.' => {
                    beams.push(d.apply(x, y));
                }
                '\\' => {
                    beams.push(
                        match d {
                            Right => Down,
                            Down => Right,
                            Left => Up,
                            Up => Left,
                        }
                        .apply(x, y),
                    );
                }
                '/' => {
                    beams.push(
                        match d {
                            Right => Up,
                            Up => Right,
                            Left => Down,
                            Down => Left,
                        }
                        .apply(x, y),
                    );
                }
                '|' if d == Direction::Right || d == Direction::Left => {
                    beams.push(Up.apply(x, y));
                    beams.push(Down.apply(x, y));
                }
                '|' if d == Direction::Up || d == Direction::Down => {
                    beams.push(d.apply(x, y));
                }
                '-' if d == Direction::Up || d == Direction::Down => {
                    beams.push(Left.apply(x, y));
                    beams.push(Right.apply(x, y));
                }
                '-' if d == Direction::Right || d == Direction::Left => {
                    beams.push(d.apply(x, y));
                }
                _ => (),
            }
        }
    }

    // println!("{energised:?}");
    return energised.len() - 1;
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);

    return energise(&grid, (0, 0, Direction::Right));
}

pub fn part2(input: &str) -> usize {
    let grid = parse(input);

    let from_left = (0..grid.len()).map(|y| (0, y, Direction::Right));
    let from_right = (0..grid.len()).map(|y| (grid[0].len() - 1, y, Direction::Left));

    let from_top = (0..grid[0].len()).map(|x| (x, 0, Direction::Down));
    let from_bottom = (0..grid.len()).map(|x| (x, grid.len() - 1, Direction::Up));

    let result = from_left
        .chain(from_right)
        .chain(from_top)
        .chain(from_bottom)
        .map(|start| energise(&grid, start))
        .max()
        .unwrap();

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
        assert_eq!(result, 7521);
    }
}
