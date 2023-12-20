use std::collections::BTreeMap;

use itertools::Itertools;

#[derive(Debug)]
enum ModuleKind {
    Broadcaster,
    Conjunction,
    FlipFlop,
}

fn parse(input: &str) -> (BTreeMap<&str, ModuleKind>, BTreeMap<&str, Vec<&str>>) {
    use ModuleKind::*;

    let nodes = input.lines().map(|line| {
        let (name_str, dest_str) = line.split_once(" -> ").unwrap();
        let destinations = dest_str.split(',').collect_vec();

        let (name, kind) = match name_str {
            s if s.starts_with("broadcaster") => (s, Broadcaster),
            s if s.starts_with("%") => (s.trim_start_matches("%"), FlipFlop),
            s if s.starts_with("&") => (s.trim_start_matches("&"), Conjunction),
            _ => unreachable!("Unexpected module kind when parsing"),
        };

        return ((name, kind), (name, destinations));
    });

    let (kinds, destinations): (BTreeMap<_, _>, BTreeMap<_, _>) = nodes.unzip();

    return (kinds, destinations);
}

pub fn part1(input: &str) -> usize {
    let (kinds, destinations) = parse(input);

    

    dbg!(&kinds);
    dbg!(&destinations);

    return 0;
}

pub fn part2(input: &str) -> usize {
    return 0;
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

    const EXAMPLE: &str = "";

    #[test]
    fn part1_example() {
        let result = part1(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(result, 32000000);
    }

    #[test]
    fn part1_example2() {
        let result = part1(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );
        assert_eq!(result, 11687500);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
