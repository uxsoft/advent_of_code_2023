use itertools::Itertools;
use regex::Regex;

fn parse(input: &str) -> Vec<Vec<&str>> {
    let double_line_ending = Regex::new("\r?\n\r?\n").unwrap();
    let mirrors = double_line_ending.split(input);

    mirrors.map(|m| m.lines().collect_vec()).collect_vec()
}

fn detect_vertical_mirror(plane: &Vec<&str>) -> usize {
    let width = plane[0].len();
    for i in 0..width - 1 {
        let slice_length = i.min(width - i - 2);

        let eq = plane.iter().all(|row| {
            let row_eq = (0..=slice_length).all(|l| {
                let left_row = row.as_bytes()[i - l];
                let right_row = row.as_bytes()[i + l + 1];

                return left_row == right_row;
            });

            return row_eq;
        });

        if eq {
            return i + 1; // 1 based index
        }
    }

    return 0;
}

fn detect_horizontal_mirror(plane: &Vec<&str>) -> usize {
    let height = plane.len();
    for i in 0..height - 1 {
        let slice_length = i.min(height - i - 2);

        let eq = (0..=slice_length).all(|l| {
            // println!("Checking i: {i} :: {slice_length}, #{} = #{} < {height}", i - l, i + 1 + l);

            let top_row = plane[i - l];
            let bottom_row = plane[i + 1 + l];

            return top_row == bottom_row;
        });

        if eq {
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
            let v = detect_vertical_mirror(m);
            let h = detect_horizontal_mirror(m);

            // println!("hscore: {h}, vscore: {v}");

            return v + 100 * h;
        })
        .sum();

    return result;
}

fn part2(input: &str) -> usize {
    return 0;
}

pub fn process(input: String) {
    let result = part1(&input);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_vm1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let example = parse(input);
        let result = detect_vertical_mirror(example.first().unwrap());
        assert_eq!(result, 5);
    }

    #[test]
    fn part1_hm1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let example = parse(input);
        let result = detect_horizontal_mirror(example.first().unwrap());
        assert_eq!(result, 0);
    }

    #[test]
    fn part1_hm2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let example = parse(input);
        let result = detect_horizontal_mirror(example.first().unwrap());
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_example() {
        let input = "#.##..##.
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
        let result = part1(input);
        assert_eq!(result, 405);
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
