use itertools::Itertools;
use regex::Regex;

fn parse(input: &str) -> Vec<Vec<&str>> {
    let double_line_ending = Regex::new("\r?\n\r?\n").unwrap();
    let mirrors = double_line_ending.split(input);

    mirrors.map(|m| m.lines().collect_vec()).collect_vec()
}

fn find_vertical_mirror(plane: &Vec<&str>, expected_differences: usize) -> usize {
    let width = plane[0].len();
    for i in 0..width - 1 {
        let slice_length = i.min(width - i - 2);

        let diff: usize = plane
            .iter()
            .map(|row| {
                let row_diff: usize = (0..=slice_length)
                    .filter(|l| {
                        let left_row = row.as_bytes()[i - l];
                        let right_row = row.as_bytes()[i + l + 1];

                        return left_row != right_row;
                    })
                    .count();

                return row_diff;
            })
            .sum();

        if diff == expected_differences {
            return i + 1; // 1 based index
        }
    }

    return 0;
}

fn find_horizontal_mirror(plane: &Vec<&str>, expected_differences: usize) -> usize {
    let height = plane.len();
    for i in 0..height - 1 {
        let slice_length = i.min(height - i - 2);

        let diff: usize = (0..=slice_length)
            .map(|l| {
                let top_row = plane[i - l];
                let bottom_row = plane[i + 1 + l];

                let diff_row = top_row
                    .chars()
                    .zip(bottom_row.chars())
                    .filter(|(a, b)| a != b)
                    .count();

                return diff_row;
            })
            .sum();

        if diff == expected_differences {
            // println!("Equal {i}: {}");
            return i + 1; // 1 based index
        }
    }

    return 0;
}

pub fn part1(input: &str) -> usize {
    let mirrors = parse(input);

    let result = mirrors
        .iter()
        .map(|m| {
            let v = find_vertical_mirror(m, 0);
            let h = find_horizontal_mirror(m, 0);

            // println!("hscore: {h}, vscore: {v}");

            return v + 100 * h;
        })
        .sum();

    return result;
}

pub fn part2(input: &str) -> usize {
    let mirrors = parse(input);

    let result = mirrors
        .iter()
        .map(|m| {
            let v = find_vertical_mirror(m, 1);
            let h = find_horizontal_mirror(m, 1);
            return v + 100 * h;
        })
        .sum();

    return result;
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

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 405);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 35521);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 400);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 34795);
    }
}
