use indicatif::ProgressIterator;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::{
        alphanumeric1 as alphanumeric, char, digit1, line_ending, multispace1, one_of,
    },
    combinator::{cut, map, map_res, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::{separated_list0, separated_list1},
    number::complete::u32,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    Err, IResult, Parser,
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

pub fn part1(input: String) {
    let (times, distances) = parse(&input);

    let races = times.iter().zip(distances.iter());

    let result: Vec<usize> = races
        
        .map(|(time, limit)| {
            (1..*time)
                .map(|hold| {
                    let speed = hold;
                    let time_remaining = time - hold;
                    let distance = speed * time_remaining;
                    // println!("Hold for {hold}, distance {distance} limit {limit}");
                    return distance;
                })
                .filter(|a| a > limit)
                .count()
        })
        .collect();
    
    println!("{:?}", result);
    println!("Result: {:?}", result.iter().product::<usize>());
}

pub fn part2(input: String) {
    let (times, distances) = parse(&input);

    let races = times.iter().zip(distances.iter());

    let result: Vec<usize> = races
        
        .map(|(time, limit)| {
            (1..*time)
                .map(|hold| {
                    let speed = hold;
                    let time_remaining = time - hold;
                    let distance = speed * time_remaining;
                    // println!("Hold for {hold}, distance {distance} limit {limit}");
                    return distance;
                })
                .filter(|a| a > limit)
                .count()
        })
        .collect();
    
    println!("{:?}", result);
    println!("Result: {:?}", result.iter().product::<usize>());
}

pub fn process(input: String) {
    part2(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_test() {
        part1(include_str!("example.txt").to_string())
    }
}
