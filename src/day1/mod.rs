pub fn part1(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .filter(|char| char.is_ascii_digit())
                .collect::<Vec<_>>();
            let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            // println!("number: {number}");
            return number.parse::<u32>().unwrap();
        })
        .sum();
    return result;
}

pub fn part2(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            let numbers = line
                .replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "th3ree")
                .replace("four", "fo4ur")
                .replace("five", "fi5ve")
                .replace("six", "s6ix")
                .replace("seven", "se7ven")
                .replace("eight", "ei8ght")
                .replace("nine", "ni9ne")
                .chars()
                .filter(|char| char.is_ascii_digit())
                .collect::<Vec<_>>();
            let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            // println!("number: {number}");
            return number.parse::<u32>().unwrap();
        })
        .sum();

    return result;
}

pub fn process(input: String) {
    let result = part2(&input);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 56042);
    }

    #[test]
    fn part2_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 55358);
    }
}
