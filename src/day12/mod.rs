use std::collections::HashMap;
use itertools::Itertools;

fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (chromosome, guards_str) = line.split_once(' ').unwrap();

            let guards = guards_str
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec();

            (chromosome, guards)
        })
        .collect_vec()
}

fn arrangements(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    chromosome: &[u8],
    current_group_size: Option<usize>,
    guards: &[usize],
) -> usize {
    if chromosome.is_empty() {
        // We're at the end
        return match (current_group_size, guards.len()) {
            // All groups satisfied and terminated
            (None, 0) => 1,
            // Last group not terminated, but correct size
            (Some(x), 1) if x == guards[0] => 1,
            // Otherwise not a valid solution
            _ => 0,
        };
    }
    if current_group_size.is_some() && guards.is_empty() {
        return 0;
    }

    let key = (
        chromosome.len(),
        current_group_size.unwrap_or(0),
        guards.len(),
    );

    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (chromosome[0], current_group_size) {
        // Terminating a group, group doesn't match the guard, so terminate this branch as it would lead to an invalid chromosome
        (b'.', Some(x)) if x != guards[0] => 0,
        // Terminating a group, group matches the guard, so clean the current group, advance guards and continue recursing
        (b'.', Some(_)) => arrangements(cache, &chromosome[1..], None, &guards[1..]),
        // Operational after an operational, so skip it
        (b'.', None) => arrangements(cache, &chromosome[1..], None, guards),
        // Another # in the group, increment the group size and continue
        (b'#', Some(g)) => arrangements(cache, &chromosome[1..], Some(g + 1), guards),
        // First #, so start a new group
        (b'#', None) => arrangements(cache, &chromosome[1..], Some(1), guards),
        // ? after a group of #, decision will be forced by group size vs guard
        (b'?', Some(x)) => {
            if x == guards[0] {
                // Forced to terminate (.)
                arrangements(cache, &chromosome[1..], None, &guards[1..])
            } else {
                // Forced to continue in the group (#)
                arrangements(cache, &chromosome[1..], Some(x + 1), guards)
            }
        }
        // We can choose if to continue (.) or start a new group (#)
        (b'?', None) => {
            arrangements(cache, &chromosome[1..], Some(1), guards)
                + arrangements(cache, &chromosome[1..], None, guards)
        }
        _ => unreachable!(),
    };
    cache.insert(key, ways);
    return ways;
}

pub fn part1(input: &str) -> usize {
    let lines = parse(input);

    let result = lines
        .iter()
        .map(|(row, guards)| 
            arrangements(&mut HashMap::new(), row.as_bytes(), None, &guards)
        )
        .sum();

    return result;
}

fn part2(input: &str) -> usize {
    let lines = parse(input);

    let result = lines
        .iter()
        .map(|(row, guards)| (std::iter::repeat(row).take(5).join("?"), guards.repeat(5)))
        .map(|(row, guards)| 
            arrangements(&mut HashMap::new(), row.as_str().as_bytes(), None, &guards)
        )
        .sum();

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

    #[test]
    fn part1_example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 7090);
    }

    #[test]
    fn part2_example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = part2(input);
        assert_eq!(result, 525152);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 6792010726878);
    }
}
