use indicatif::ProgressIterator;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

pub fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    pub fn number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }

    let mut parser = separated_pair(
        preceded(
            tag("Time:"),
            preceded(multispace1, separated_list1(multispace1, number)),
        ),
        line_ending,
        preceded(
            tag("Distance:"),
            preceded(multispace1, separated_list1(multispace1, number)),
        ),
    );

    let (_, result) = parser.parse(input).unwrap();

    return result;
}

fn winning_races_brute_force(time: u64, record_distance: u64) -> usize {
    let result = (1..time)
        .progress_count(time)
        .map(|hold| {
            let speed = hold;
            let time_remaining = time - hold;
            let distance = speed * time_remaining;
            // println!("Hold for {hold}, distance {distance} limit {limit}");
            return distance;
        })
        .filter(|a| a > &record_distance)
        .count();

    return result;
}

/// distance traveled is equal to total time - time pressed, times time pressed
///
/// total time: t
/// time pressed: x
/// recordDistance = d
///
/// (t - x) * x = d => -x^2 + xt - d = 0
///
/// equation solutions are min and max time pressed to beat record
fn winning_races_equation(time: u64, record_distance: u64) -> usize {
    let t = time as f64;
    let d = record_distance as f64;

    // * D = b^2 - 4ac
    //  * so, t^2 - 4d
    let D_root = (t * t - 4. * d).sqrt();

    // * x_1,2 = (-b +- sqrt of D)/2
    let a = (t - D_root) / 2.;
    let b = (t + D_root) / 2.;

    let min_time = (a + 0.0001).ceil() as usize;
    let max_time = (b - 0.0001).floor() as usize;

    return max_time - min_time + 1;
}

pub fn part1(input: &str) -> usize {
    let (times, distances) = parse(&input);

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| (*time, *distance))
        .collect_vec();

    let result = races
        .iter()
        .map(|(t, d)| winning_races_equation(*t, *d))
        .product::<usize>();

    return result;
}

pub fn part2(input: &str) -> usize {
    let (times, distances) = parse(&input);

    let time: u64 = times.iter().join("").parse().unwrap();
    let distance: u64 = distances.iter().join("").parse().unwrap();

    let races = vec![(time, distance)];

    let result = races
        .iter()
        .map(|(t, d)| winning_races_equation(*t, *d))
        .product::<usize>();

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

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 500346);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 71503);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 42515755);
    }
}
