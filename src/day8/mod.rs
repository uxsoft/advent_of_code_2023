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

pub fn part1(input: String) {
    let (directions, nodes) = parse(&input);

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

    println!("Result: {i}");
}

pub fn part2_bf(input: String) {
    let (directions, nodes) = parse(&input);

    let mut i: u64 = 0;
    let mut current_nodes: Vec<&str> = nodes
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| *n)
        .collect();

    for direction in directions.chars().cycle() {
        // let (l, r) = nodes.get(current_node).unwrap();

        current_nodes = current_nodes
            .iter()
            .map(|n| match direction {
                'L' => nodes.get(*n).unwrap().0,
                _ => nodes.get(*n).unwrap().1,
            })
            .collect();

        i += 1;

        if current_nodes.iter().all(|n| n.ends_with('Z')) {
            break;
        }

        if i % 10_000_000 == 0 {
            println!("{i}");
        }
    }

    println!("Result: {i}");
}

pub fn part2_trends(input: String) {
    let (directions, nodes) = parse(&input);

    let mut cycles: Vec<u64> = vec![];

    for starting_node in nodes.keys().filter(|n| n.ends_with('A')) {
        let mut i: usize = 0;
        let mut current_node = starting_node;
        let mut trends: Vec<(usize, usize)> = vec![];

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
                let new_trend = (direction_pos, delta);
                if trends.contains(&new_trend) {
                    trends.push(new_trend);
                    cycles.push(delta as u64);
                    println!("[{starting_node}]: {trends:?}");
                    break;
                } else {
                    trends.push(new_trend);
                }
            }
        }
    }
    let lcm = reikna::factor::lcm_all(&cycles);

    println!("Result: {lcm}");
    // Then LCM (trends)
}

pub fn process(input: String) {
    part2_trends(input)
}
