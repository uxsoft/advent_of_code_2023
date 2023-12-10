use std::collections::BTreeMap;
use regex::Regex;

fn parse(input: &str) -> (&str, BTreeMap<&str, (&str, &str)>) {
    let lines: Vec<&str> = input.lines().collect();
    let instructions = lines.first().unwrap();

    let re = Regex::new(r"([\w]+) = \(([\w]+), ([\w]+)\)").unwrap();

    let map: BTreeMap<&str, (&str, &str)> = lines
        .iter()
        .skip(2)
        .map(|line| {
            let matches = re.captures(line).unwrap();
            return (
                matches.get(1).unwrap().as_str(),
                (
                    matches.get(2).unwrap().as_str(),
                    matches.get(3).unwrap().as_str(),
                ),
            );
        })
        .collect();

    return (instructions, map);
}

pub fn part1(input: &str) -> u64 {
    let (directions, nodes) = parse(input);

    let mut i = 0;
    let mut current_node = "AAA";

    for direction in directions.chars().cycle() {
        let (l, r) = nodes.get(current_node).unwrap();

        match direction {
            'L' => current_node = l,
            'R' => current_node = r,
            other => panic!("Unexpected direction, encountered {other}"),
        }

        i += 1;

        if current_node == "ZZZ" {
            break;
        }
    }

    return i;
}

pub fn part2(input: &str) -> u64{
    let (directions, nodes) = parse(input);

    let mut cycles: Vec<u64> = vec![];

    for starting_node in nodes.keys().filter(|n| n.ends_with('A')) {
        let mut i: u64 = 0;
        let mut current_node = starting_node;
        let mut trends: Vec<(u64, u64)> = vec![];

        for (direction_pos, direction) in directions.chars().enumerate().cycle() {
            let (l, r) = nodes.get(current_node).unwrap();

            current_node = match direction {
                'L' => l,
                'R' => r,
                _ => r,
            };

            i += 1;

            if current_node.ends_with('Z') {
                let (_, last) = trends.last().unwrap_or(&(0, 0));
                let delta = i - last;
                let new_trend = (direction_pos as u64, delta);
                if trends.contains(&new_trend) {
                    trends.push(new_trend);
                    cycles.push(delta);
                    println!("[{starting_node}]: {trends:?}");
                    break;
                } else {
                    trends.push(new_trend);
                }
            }
        }
    }
    let lcm = reikna::factor::lcm_all(&cycles);

    return lcm;
}

pub fn process(input: String) {
    let result = part2(&input);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn part1_example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 12737);
    }

    #[test]
    fn part2_example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 9064949303801);
    }
}