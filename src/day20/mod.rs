use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleKind {
    Broadcaster,
    Conjunction,
    FlipFlop,
}

impl ModuleKind {
    fn process() {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SignalKind {
    Low,
    High,
}

fn parse(input: &str) -> (BTreeMap<&str, ModuleKind>, BTreeMap<&str, Vec<&str>>) {
    use ModuleKind::*;

    let nodes = input.lines().map(|line| {
        let (name_str, dest_str) = line.split_once(" -> ").unwrap();
        let destinations = dest_str.split(", ").collect_vec();

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

fn extract_result(factors: BTreeMap<&str, Vec<usize>>) -> usize {
    let first_encounters = factors.values().map(|v| *v.first().unwrap()).collect_vec();
    dbg!(factors);
    println!("Got: {:?}", first_encounters);
    println!("Expected: 3917, 3919, 4007, 4027");
    let res2 = reikna::factor::lcm_all(&first_encounters.iter().map(|i| *i as u64).collect_vec());

    return 0;//res2 as usize;
}

pub fn part2(input: &str) -> usize {
    let (kinds, destinations) = parse(input);

    let mut queue: VecDeque<(&str, &str, SignalKind)> = VecDeque::new();

    let mut ff_memory = BTreeMap::new();
    let mut con_memory: BTreeMap<&str, BTreeMap<&str, SignalKind>> = destinations
        .keys()
        .filter(|n| kinds.get(**n).unwrap() == &ModuleKind::Conjunction)
        .map(|n| {
            let mem = destinations
                .iter()
                .filter(|(_, dl)| dl.contains(n))
                .map(|(dn, _)| (*dn, SignalKind::Low))
                .collect::<BTreeMap<&str, SignalKind>>();
            (*n, mem)
        })
        .collect();

    let mut factors: BTreeMap<&str, Vec<usize>> = BTreeMap::new();

    for i in 1.. {
        queue.push_back(("button", "broadcaster", SignalKind::Low));

        while let Some((sender, receiver, signal)) = queue.pop_front() {
            // println!("{sender} -{signal:?}-> `{receiver}`");

            if !kinds.contains_key(receiver) {
                if signal == SignalKind::Low {
                    return i;
                } else {
                    continue;
                }
            }

            for mon in vec!["mp", "qt", "qb", "ng"] {
                if receiver == mon && signal == SignalKind::Low {
                    println!("{i}: Encountered LOW to {mon}");
                    factors
                        .entry(receiver)
                        .and_modify(|v| v.push(i))
                        .or_insert(vec![i]);
                }
            }

            if factors.len() == 4 && factors.values().all(|v| v.len() > 2) {
                return extract_result(factors);
            }

            match kinds.get(receiver).unwrap() {
                ModuleKind::Broadcaster => {
                    for destination in &destinations[receiver] {
                        queue.push_back((receiver, destination, signal));
                    }
                }
                ModuleKind::FlipFlop => {
                    let is_on = ff_memory.entry(receiver).or_insert(false);
                    match (is_on, signal) {
                        (false, SignalKind::Low) => {
                            ff_memory.insert(receiver, true);
                            for destination in &destinations[receiver] {
                                queue.push_back((receiver, destination, SignalKind::High));
                            }
                        }
                        (true, SignalKind::Low) => {
                            ff_memory.insert(receiver, false);
                            for destination in &destinations[receiver] {
                                queue.push_back((receiver, destination, SignalKind::Low));
                            }
                        }
                        (_, SignalKind::High) => (), // High pulse when off is ignored
                    }
                }
                ModuleKind::Conjunction => {
                    let state = con_memory.entry(receiver).or_insert(BTreeMap::new());
                    state.insert(sender, signal);

                    let all_high = state.values().all(|v| v == &SignalKind::High);
                    for destination in &destinations[receiver] {
                        queue.push_back((
                            receiver,
                            destination,
                            if all_high {
                                SignalKind::Low
                            } else {
                                SignalKind::High
                            },
                        ));
                    }
                }
            }
        }
    }

    dbg!(&factors);
    return 1;
}

pub fn part1(input: &str) -> usize {
    let (kinds, destinations) = parse(input);

    let mut queue: Vec<(&str, &str, SignalKind)> = vec![];

    let mut low_counter = 0;
    let mut high_counter = 0;

    let mut ff_memory = BTreeMap::new();
    let mut con_memory: BTreeMap<&str, BTreeMap<&str, SignalKind>> = destinations
        .keys()
        .filter(|n| kinds.get(**n).unwrap() == &ModuleKind::Conjunction)
        .map(|n| {
            let mem = destinations
                .iter()
                .filter(|(_, dl)| dl.contains(n))
                .map(|(dn, _)| (*dn, SignalKind::Low))
                .collect::<BTreeMap<&str, SignalKind>>();
            (*n, mem)
        })
        .collect();

    for _ in 0..1000 {
        queue.push(("button", "broadcaster", SignalKind::Low));

        while let Some((sender, receiver, signal)) = queue.pop() {
            println!("{sender} -{signal:?}-> `{receiver}`");

            match signal {
                SignalKind::Low => low_counter += 1,
                SignalKind::High => high_counter += 1,
            }

            if !kinds.contains_key(receiver) {
                println!("Receiver {receiver} not found");
                continue;
            }

            match kinds.get(receiver).unwrap() {
                ModuleKind::Broadcaster => {
                    for destination in &destinations[receiver] {
                        queue.push((receiver, destination, signal));
                    }
                }
                ModuleKind::FlipFlop => {
                    let is_on = ff_memory.entry(receiver).or_insert(false);
                    match (is_on, signal) {
                        (false, SignalKind::Low) => {
                            ff_memory.insert(receiver, true);
                            for destination in &destinations[receiver] {
                                queue.push((receiver, destination, SignalKind::High));
                            }
                        }
                        (true, SignalKind::Low) => {
                            ff_memory.insert(receiver, false);
                            for destination in &destinations[receiver] {
                                queue.push((receiver, destination, SignalKind::Low));
                            }
                        }
                        (_, SignalKind::High) => (), // High pulse when off is ignored
                    }
                }
                ModuleKind::Conjunction => {
                    let state = con_memory.entry(receiver).or_insert(BTreeMap::new());
                    state.insert(sender, signal);

                    let all_high = state.values().all(|v| v == &SignalKind::High);
                    for destination in &destinations[receiver] {
                        queue.push((
                            receiver,
                            destination,
                            if all_high {
                                SignalKind::Low
                            } else {
                                SignalKind::High
                            },
                        ));
                    }
                }
            }
        }
    }

    return low_counter * high_counter;
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
        assert_eq!(result, 919383692);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 247702167614647);
    }
}
