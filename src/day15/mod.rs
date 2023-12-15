use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug)]
enum Operation {
    Remove,
    Set,
}

#[derive(Debug)]
struct Step {
    label: String,
    box_number: usize,
    operation: Operation,
    focal_length: usize,
}

lazy_static! {
    static ref STEP_REGEX: Regex = Regex::new(r"([\w]+)(\-|\=)([\d])*").unwrap();
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = STEP_REGEX.captures(s).unwrap();

        Ok(Self {
            label: captures.get(1).unwrap().as_str().to_string(),
            box_number: hash(captures.get(1).unwrap().as_str()),
            operation: match captures.get(2).unwrap().as_str() {
                "=" => Operation::Set,
                "-" => Operation::Remove,
                _ => panic!("Unexpected operation"),
            },
            focal_length: captures
                .get(3)
                .map(|c| c.as_str().parse::<usize>().unwrap_or(0))
                .unwrap_or(0),
        })
    }
}

fn hash(input: &str) -> usize {
    let mut current_value: usize = 0;

    for c in input.as_bytes() {
        current_value += *c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    return current_value;
}

pub fn part1(input: &str) -> usize {
    let result = input.split(',').map(|p| hash(p)).sum();
    return result;
}

pub fn part2(input: &str) -> usize {
    let steps: Vec<Step> = input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect_vec();

    let mut boxes: BTreeMap<usize, Vec<Step>> = (0..=256).map(|i| (i, Vec::new())).collect();

    for step in steps {
        match step.operation {
            Operation::Remove => {
                let b = boxes.get_mut(&step.box_number).unwrap();
                if let Some((i, _)) = b.iter().find_position(|lens| lens.label == step.label) {
                    b.remove(i);
                }
            }
            Operation::Set => {
                let b = boxes
                    .get_mut(&step.box_number)
                    .expect(&format!("Expected a box with {}", &step.box_number));
                if let Some((i, _)) = b.iter().find_position(|lens| lens.label == step.label) {
                    b.remove(i);
                    b.insert(i, step);
                } else {
                    b.push(step);
                }
            }
        }
    }

    let result: usize = boxes
        .iter()
        .map(|(bi, b)| {
            b.iter()
                .enumerate()
                .map(|(li, l)| (bi + 1) * (li + 1) * l.focal_length)
                .sum::<usize>()
        })
        .sum();

    // println!("{:?}", &boxes);

    return result;
}

pub fn process(input: String) {
    use std::time::Instant;
    let now = Instant::now();
    let result = part1(&input);
    println!("Result: {result}");
    println!("Finished in: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_hash() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 1320);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 509167);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 145);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 259333);
    }
}
