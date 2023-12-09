use itertools::Itertools;

fn parse(input: String) -> Vec<Vec<i64>> {
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

fn part1(input: String) {
    let mut series = parse(input);

    let next_numbers: Vec<i64> = series.iter().map(|s| extrapolate(s)).collect();

    let result: i64 = next_numbers.iter().sum();

    println!("Result: {result}");
}

fn part2(input: String) {
    let mut series = parse(input);

    for serie in series.iter_mut() {
        serie.reverse();
    }

    let next_numbers: Vec<i64> = series.iter()
        .map(|s| extrapolate(s)).collect();

    let result: i64 = next_numbers.iter().sum();

    println!("Result: {result}");
}

pub fn process(input: String) {
    part2(input)
}
