use itertools::Itertools;
use regex::Regex;

fn parse(input: &str) -> Vec<Vec<&str>> {
    let double_line_ending = Regex::new("\r?\n\r?\n").unwrap();
    let mirrors = double_line_ending.split(input);

    mirrors.map(|m| m.lines().collect_vec()).collect_vec()
}

fn detect_mirror_h(plane: &Vec<&str>) -> usize {
    for i in 1..plane[0].len() - 2 {
        let has_mirror = plane.iter().all(|line| {
            let slice_length = i.max(plane[0].len() / 2);
            let equal = line[i - slice_length..i] == line[i..i + slice_length];

            return equal;
        });

        if has_mirror {
            return i;
        }
    }

    return 0;
}

fn detect_mirror_v(plane: &Vec<&str>) -> usize {
    let mut result = 0;

    for i in 1..plane.len() - 2 {
        // plane[i] must equal plane[i+1], [i-1]==[i+2], ...

        let slice_len = i.min(plane.len() - i);

        let eq = plane[i - slice_len..i] == plane[i + 1..i + slice_len];
        if eq {
            return i;
        }
    }

    return 0;
}

pub fn part1(input: &str) -> usize {
    let mirrors = parse(input);

    let result = mirrors
        .iter()
        .map(|m| {
            let h = detect_mirror_h(m);
            let v = detect_mirror_v(m);
            return h + 100 * v;
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

    // #[test]
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
