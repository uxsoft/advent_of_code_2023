pub fn part1(input: &str) -> usize {

    return 0;
}

fn part2(input: &str) -> usize {

    return 0;
}

pub fn process(input: String) {
    let result = part1(&input);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_example() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}