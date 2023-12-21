use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq)]
enum GardenPos {
    Plot,
    Rock,
    Start,
}

fn parse(input: &str) -> Vec<Vec<GardenPos>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => GardenPos::Rock,
                    'S' => GardenPos::Start,
                    '.' => GardenPos::Plot,
                    _ => unreachable!("Unknown cell type."),
                })
                .collect_vec()
        })
        .collect_vec()
}

fn successors_inf(x: i64, y: i64, grid: &Vec<Vec<GardenPos>>) -> Vec<(i64, i64)> {
    vec![(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
        .into_iter()
        .filter(|(x, y)| {
            match grid
                .get(y.rem_euclid(grid.len() as i64) as usize)
                .and_then(|row| row.get(x.rem_euclid(row.len() as i64) as usize))
            {
                Some(GardenPos::Plot) => true,
                Some(GardenPos::Start) => true,
                _ => false,
            }
        })
        .collect_vec()
}

fn bfs(grid: &Vec<Vec<GardenPos>>, start: (usize, usize), steps: usize) -> usize {
    let mut positions = BTreeSet::from([(start.0 as i64, start.1 as i64)]);
    let mut cycles = Vec::new();

    for i in 1..=steps {
        positions = positions
            .into_iter()
            .flat_map(|(x, y)| successors_inf(x, y, grid))
            .collect();

        if i % grid.len() == 65 { // Magic number for input
            cycles.push(positions.len());
        }

        if cycles.len() > 2 {
            return quad_regression(steps / grid.len(), cycles);
        }
    }

    return positions.len();
}

fn _display(grid: &Vec<Vec<GardenPos>>, distances: BTreeMap<(usize, usize), usize>) -> String {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| match distances.get(&(x, y)) {
                    Some(i) if i < &7 => i.to_string(),
                    None => "#".to_string(),
                    _ => ".".to_string(),
                })
                .join("")
        })
        .join("\n")
}

pub fn part1(input: &str, steps: usize) -> usize {
    let grid = parse(input);

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if c == &GardenPos::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let result = bfs(&grid, start, steps);

    return result;
}

fn quad_regression(x: usize, a: Vec<usize>) -> usize {
    let d0 = a[0];
    let d1 = a[1] - a[0];
    let d2 = a[2] - a[1];
    return d0 + d1 * x + (x * (x - 1) / 2) * (d2 - d1);
}

pub fn part2(input: &str, steps: usize) -> usize {
    let grid = parse(input);

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if c == &GardenPos::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let result = bfs(&grid, start, steps);

    return result;
}

pub fn process(input: String) {
    use std::time::Instant;
    let now = Instant::now();
    let result = part2(&input, 26501365);
    println!("Result: {result}");
    println!("Finished in: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input, 64);
        assert_eq!(result, 3671);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE, 6), 16);
        assert_eq!(part2(EXAMPLE, 10), 50);
        assert_eq!(part2(EXAMPLE, 50), 1594);
        assert_eq!(part2(EXAMPLE, 100), 6536);
        assert_eq!(part2(EXAMPLE, 500), 167004);
        // assert_eq!(part2(EXAMPLE, 1000), 668697);
        // assert_eq!(part2(EXAMPLE, 5000), 16733044);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input, 26501365);
        assert_eq!(result, 609708004316870);
    }
}
