use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

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

fn successors(x: usize, y: usize, grid: &Vec<Vec<GardenPos>>) -> Vec<(usize, usize)> {
    vec![
        Some((x, y + 1)),
        y.checked_sub(1).map(|y| (x, y)),
        Some((x + 1, y)),
        x.checked_sub(1).map(|x| (x, y)),
    ]
    .into_iter()
    .flatten()
    .filter(|(x, y)| match grid.get(*y).and_then(|row| row.get(*x)) {
        Some(GardenPos::Plot) => true,
        Some(GardenPos::Start) => true,
        _ => false,
    })
    .collect_vec()
}

fn bfs(grid: &Vec<Vec<GardenPos>>, start: (usize, usize), steps: usize) -> Vec<(usize, usize)> {
    let mut positions = vec![start];

    for i in 0..steps {
        let new_positions = positions
            .iter()
            .flat_map(|(x, y)| successors(*x, *y, grid))
            .unique()
            .collect_vec();
        positions = new_positions;
    }

    return positions
}

fn dijkstra(grid: &Vec<Vec<GardenPos>>, start: &(usize, usize)) -> BTreeMap<(usize, usize), usize> {
    let result: std::collections::HashMap<(usize, usize), ((usize, usize), i32)> =
        pathfinding::directed::dijkstra::dijkstra_all(start, |&(x, y)| {
            vec![
                Some((x, y + 1)),
                y.checked_sub(1).map(|y| (x, y)),
                Some((x + 1, y)),
                x.checked_sub(1).map(|x| (x, y)),
            ]
            .iter()
            .flatten()
            .filter(|(x, y)| match grid.get(*y).and_then(|row| row.get(*x)) {
                Some(GardenPos::Plot) => true,
                _ => false,
            })
            .map(|(x, y)| ((*x, *y), 1))
            .collect_vec()
        });

    return result.iter().map(|(k, (_, v))| (*k, *v as usize)).collect();
}

fn display(grid: &Vec<Vec<GardenPos>>, distances: BTreeMap<(usize, usize), usize>) -> String {
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

    let positions = bfs(&grid, start, steps);


    // let within_reach = result.values().filter(|(_, v)| *v == steps as i32).count();

    return positions.iter().count();
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

    let positions = bfs(&grid, start, steps);

    return positions.iter().count();
}

pub fn process(input: String) {
    use std::time::Instant;
    let now = Instant::now();
    let result = part1(&input, 64);
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

    // #[test]
    fn part1_example() {
        let result = part1(EXAMPLE, 6);
        assert_eq!(result, 16);
    }

    // #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input, 64);
        assert_eq!(result, 3671);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE, 5000);
        assert_eq!(result, 16733044);
    }

    // #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input, 26501365);
        assert_eq!(result, 0);
    }
}
