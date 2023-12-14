use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn extrapolate(input: &Vec<i64>) -> i64 {
    let sub_series: Vec<_> = input.windows(2).map(|w| w[1] - w[0]).collect();
    if sub_series.iter().all(|i| i == &0) {
        return *input.last().unwrap();
    } else {
        let sub = extrapolate(&sub_series);
        return input.last().unwrap() + sub;
    }
}

fn part1(input: &str) -> i64 {
    let mut series = parse(input);

    let next_numbers: Vec<i64> = series.iter().map(|s| extrapolate(s)).collect();

    let result: i64 = next_numbers.iter().sum();

    return result;
}

pub fn part2(input: &str) -> i64 {
    let mut series = parse(input);

    for serie in series.iter_mut() {
        serie.reverse();
    }

    let next_numbers: Vec<i64> = series.iter()
        .map(|s| extrapolate(s)).collect();

    let result: i64 = next_numbers.iter().sum();

    return result;
}

pub fn process(input: String) {
    let result = part2(&input);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part1(input);
        assert_eq!(result, 114);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 1666172641);
    }

    #[test]
    fn part2_example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part2(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 933);
    }
}