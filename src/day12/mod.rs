use itertools::Itertools;

enum Field {
    Spring,
    Separator,
    Unknown,
}

fn parse(input: &str) -> Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let line_parts = line.split(' ').collect_vec();

            let groups = line_parts
                .first()
                .unwrap()
                .split('.')
                .filter(|group| !group.is_empty())
                .collect_vec();

            let springs_counts = groups
                .iter()
                .map(|g| g.chars().filter(|c| c == &'#').count())
                .collect_vec();

            let unknown_counts = groups
                .iter()
                .map(|g| g.chars().filter(|c| c == &'?').count())
                .collect_vec();

            let guards = line_parts
                .last()
                .unwrap()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec();

            (springs_counts, unknown_counts, guards)
        })
        .collect_vec()
}

pub fn part1(input: &str) -> usize {
    let lines = parse(input);

    let result = lines
        .iter()
        .map(|(spring_counts, unknown_counts, guards)| {
            let combinations = spring_counts
                .iter()
                .zip(unknown_counts)
                .zip(guards)
                .map(|((springs, unknowns), guard)| {
                    return springs + unknowns - guard;
                })
                .sum::<usize>();
            println!("Line: {combinations}");
            return combinations;
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
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_example() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
