use std::collections::HashSet;

use itertools::Itertools;

pub fn parse(input: &str) -> HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                '.' => None,
                c => panic!("Unexpected char in input {c}"),
            })
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    part2(input, 2)
}

pub fn part2(input: &str, factor: usize) -> usize {
    let galaxies = parse(input);
    let dim_y = input.lines().count();
    let dim_x = input.lines().nth(0).unwrap().chars().count();

    let expansions_y = (0..dim_y)
        .filter(|y| !galaxies.iter().any(|(_, gy)| gy == y))
        .collect_vec();
    let expansions_x = (0..dim_x)
        .filter(|x| !galaxies.iter().any(|(gx, _)| gx == x))
        .collect_vec();

    // println!("Expansions: {:?} + {:?}", expansions_x, expansions_y);

    let expanded_galaxies = galaxies
        .iter()
        .map(|(gx, gy)| {
            let ey = expansions_y.iter().filter(|y| gy > y).count() * (factor - 1);
            let ex = expansions_x.iter().filter(|x| gx > x).count() * (factor - 1);
            (gx + ex, gy + ey)
        })
        .collect::<HashSet<_>>();

    let distances = expanded_galaxies
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|((ax, ay), (bx, by))| {
            let distance = ax.abs_diff(*bx) + ay.abs_diff(*by);
            // println!("Distance between {ax},{ay} and {bx},{by} : {distance}");
            return distance;
        })
        .sum();

    return distances;
}

pub fn process(input: String) {
    let result = part2(&input, 1_000_000);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part1(input);
        assert_eq!(result, 374);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 9681886);
    }

    #[test]
    fn part2_example1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part2(input, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn part2_example2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part2(input, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input, 1_000_000);
        assert_eq!(result, 791134099634);
    }
}
