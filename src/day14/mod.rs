use std::{collections::HashMap, iter};

use indicatif::ProgressIterator;
use itertools::Itertools;

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        Grid { grid }
    }

    fn tilt_north(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let 'O' = self.grid[y][x] {
                    // Try roll north
                    let mut y2 = y;
                    while y2 > 0 && self.grid[y2 - 1][x] == '.' {
                        y2 -= 1;
                    }

                    self.grid[y][x] = '.';
                    self.grid[y2][x] = 'O';
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for x in 0..self.grid[0].len() {
            for y in 0..self.grid.len() {
                if let 'O' = self.grid[y][x] {
                    // Try roll north
                    let mut x2 = x;
                    while x2 > 0 && self.grid[y][x2 - 1] == '.' {
                        x2 -= 1;
                    }

                    self.grid[y][x] = '.';
                    self.grid[y][x2] = 'O';
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for ry in 1..=self.grid.len() {
            let y = self.grid.len() - ry;
            for x in 0..self.grid[y].len() {
                if let 'O' = self.grid[y][x] {
                    // Try roll north
                    let mut y2 = y;
                    while y2 < self.grid.len() - 1 && self.grid[y2 + 1][x] == '.' {
                        y2 += 1;
                    }

                    self.grid[y][x] = '.';
                    self.grid[y2][x] = 'O';
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for rx in 1..=self.grid[0].len() {
            let x = self.grid.len() - rx;
            for y in 0..self.grid.len() {
                if let 'O' = self.grid[y][x] {
                    // Try roll north
                    let mut x2 = x;
                    while x2 < self.grid[0].len() - 1 && self.grid[y][x2 + 1] == '.' {
                        x2 += 1;
                    }

                    self.grid[y][x] = '.';
                    self.grid[y][x2] = 'O';
                }
            }
        }
    }

    fn north_load(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, row)| row.iter().filter(|c| c == &&'O').count() * (self.grid.len() - y))
            .sum()
    }

    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|line| line.iter().collect::<String>())
            .join("\n")
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.tilt_north();

    let result = grid.north_load();

    println!("{}", grid.to_string());

    return result;
}

fn part2(input: &str) -> usize {
    let iterations = 1000000000;
    let mut grid = Grid::parse(input);

    let mut cycle_detector: HashMap<String, usize> = HashMap::new();
    let mut cycle_predictor: HashMap<usize, usize> = HashMap::new();

    let first_cycle_index = (1..=1_000_000_000).find_map(|i| {
        grid.tilt_north();
        grid.tilt_west();
        grid.tilt_south();
        grid.tilt_east();

        let state = grid.to_string();
        cycle_predictor.insert(i, grid.north_load());
        if let Some(before) = cycle_detector.get(&state) {
            println!("Detected a cycle: {before}, {i}");
            Some((*before, i))
        } else {
            cycle_detector.insert(state, i);
            None
        }
    });

    if let Some((first_encounter, second_encounter)) = first_cycle_index {
        let cycle_length = second_encounter - first_encounter;
        let cycle_i = first_encounter + (iterations - first_encounter) % cycle_length;
        
        let result = cycle_predictor.get(&cycle_i).unwrap();
        println!("Predicting a result {result} based on {cycle_i}");

        return *result;
    }
    return 0;
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

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 136);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 106378);
    }

    #[test]
    fn part2_example() {
        let input = EXAMPLE;
        let result = part2(input);
        assert_eq!(result, 64);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
