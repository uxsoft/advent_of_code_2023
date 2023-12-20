use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind {
    Broadcaster,
    Conjunction { memory: BTreeMap<String, Pulse> },
    FlipFlop { is_on: bool },
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    kind: ModuleKind,
    destinations: Vec<String>,
}

type Signal<'a> = (String, String, Pulse);

impl Module {
    fn is_conjunction(&self) -> bool {
        match self.kind {
            ModuleKind::Conjunction { .. } => true,
            _ => false,
        }
    }

    fn receive_pulse(&mut self, sender: String, pulse: Pulse) -> Vec<Signal> {
        match self.kind {
            ModuleKind::Broadcaster => self
                .destinations
                .iter()
                .map(|destination| (self.name.clone(), destination.clone(), pulse))
                .collect_vec(),
            ModuleKind::FlipFlop { ref mut is_on } => {
                match (&is_on, pulse) {
                    (false, Pulse::Low) => {
                        *is_on = true;

                        self.destinations
                            .iter()
                            .map(|destination| {
                                (self.name.clone(), destination.clone(), Pulse::High)
                            })
                            .collect_vec()
                    }
                    (true, Pulse::Low) => {
                        *is_on = false;

                        self.destinations
                            .iter()
                            .map(|destination| (self.name.clone(), destination.clone(), Pulse::Low))
                            .collect_vec()
                    }
                    (_, Pulse::High) => vec![], // High pulse when off is ignored
                }
            }
            ModuleKind::Conjunction { ref mut memory } => {
                memory.insert(sender, pulse);

                let all_high = memory.values().all(|v| v == &Pulse::High);
                let next_pulse = if all_high { Pulse::Low } else { Pulse::High };

                self.destinations
                    .iter()
                    .map(|destination| (self.name.clone(), destination.clone(), next_pulse))
                    .collect_vec()
            }
        }
    }

    fn init_memory(&mut self, modules: &Vec<Module>) {
        match self.kind {
            ModuleKind::Conjunction { ref mut memory } => modules
                .iter()
                .filter(|m| m.destinations.contains(&self.name))
                .for_each(|m| {
                    memory.insert(m.name.clone(), Pulse::Low);
                }),
            _ => (),
        }
    }
}

fn parse(input: &str) -> (BTreeMap<&str, ModuleKind>, BTreeMap<&str, Vec<&str>>) {
    use ModuleKind::*;

    let nodes = input.lines().map(|line| {
        let (name_str, dest_str) = line.split_once(" -> ").unwrap();
        let destinations = dest_str.split(", ").collect_vec();

        let (name, kind) = match name_str {
            s if s.starts_with("broadcaster") => (s, Broadcaster),
            s if s.starts_with("%") => (s.trim_start_matches("%"), FlipFlop { is_on: false }),
            s if s.starts_with("&") => (
                s.trim_start_matches("&"),
                Conjunction {
                    memory: BTreeMap::new(),
                },
            ),
            _ => unreachable!("Unexpected module kind when parsing"),
        };

        return ((name, kind), (name, destinations));
    });

    let (kinds, destinations): (BTreeMap<_, _>, BTreeMap<_, _>) = nodes.unzip();

    return (kinds, destinations);
}

fn parse_modules(input: &str) -> BTreeMap<String, Module> {
    use ModuleKind::*;

    input
        .lines()
        .map(|line| {
            let (name_str, dest_str) = line.split_once(" -> ").unwrap();
            let destinations = dest_str.split(", ").map(str::to_string).collect_vec();

            let (name, kind) = match name_str {
                s if s.starts_with("broadcaster") => (s.to_string(), Broadcaster),
                s if s.starts_with("%") => (
                    s.trim_start_matches("%").to_string(),
                    FlipFlop { is_on: false },
                ),
                s if s.starts_with("&") => (
                    s.trim_start_matches("&").to_string(),
                    Conjunction {
                        memory: BTreeMap::new(),
                    },
                ),
                _ => unreachable!("Unexpected module kind when parsing"),
            };

            (
                name.clone(),
                Module {
                    name,
                    kind,
                    destinations,
                },
            )
        })
        .collect()
}

pub fn part2(input: &str) -> usize {
    let mut modules = parse_modules(input);

    // Init memory
    let module_view = modules.values().cloned().collect_vec();
    for module in modules.values_mut() {
        module.init_memory(&module_view);
    }

    // Find the target nodes
    let monitored_nodes = modules
        .values()
        .filter(|m| m.destinations.contains(&"rx".to_string()))
        .flat_map(|target| {
            modules
                .values()
                .filter(|m| m.destinations.contains(&target.name))
                .map(|m| m.name.clone())
        })
        .collect_vec();

    let mut queue: VecDeque<Signal> = VecDeque::new();

    let mut factors: BTreeMap<String, u64> = BTreeMap::new();

    for i in 1.. {
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            for mon in monitored_nodes.iter() {
                if &receiver == mon && pulse == Pulse::Low {
                    if !factors.contains_key(mon) {
                        factors.insert(mon.clone(), i as u64);
                    }
                }
            }

            modules.entry(receiver).and_modify(|m| {
                let responses = m.receive_pulse(sender, pulse);
                queue.extend(responses);
            });
        }

        if factors.len() == 4 {
            break;
        }
    }

    let res = reikna::factor::lcm_all(&factors.into_values().collect_vec());
    return res as usize;
}

pub fn part1(input: &str) -> usize {
    let mut modules = parse_modules(input);

    // Init memory
    let module_view = modules.values().cloned().collect_vec();
    for module in modules.values_mut() {
        module.init_memory(&module_view);
    }

    let mut queue: VecDeque<Signal> = VecDeque::new();

    let mut low_counter = 0;
    let mut high_counter = 0;

    for _ in 0..1000 {
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_counter += 1,
                Pulse::High => high_counter += 1,
            }

            modules.entry(receiver).and_modify(|m| {
                let responses = m.receive_pulse(sender, pulse);
                queue.extend(responses);
            });
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
