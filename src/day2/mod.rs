struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl Game {
    pub fn parse(input: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for dices in input.split(", ") {
            let split = dices.split_ascii_whitespace().collect::<Vec<_>>();
            let count: u32 = split.first().unwrap().parse().unwrap();
            match split.last().unwrap() {
                &"red" => red = count,
                &"green" => green = count,
                &"blue" => blue = count,
                _ => (),
            }
        }

        return Self { red, green, blue };
    }

    pub fn check(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}

pub fn part2(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|row| {
            let split = row.split(": ").collect::<Vec<_>>();

            let games = split
                .last()
                .unwrap()
                .split(";")
                .map(Game::parse)
                .collect::<Vec<_>>();

            let max_game = games.iter().fold(Game::default(), |a, b| Game {
                red: a.red.max(b.red),
                blue: a.blue.max(b.blue),
                green: a.green.max(b.green),
            });

            return max_game.red * max_game.green * max_game.blue;
        })
        .sum();

    return result;
}

pub fn part1(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|row| {
            let split1 = row.split(": ").collect::<Vec<_>>();
            let game_id: u32 = split1
                .first()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let games = split1
                .last()
                .unwrap()
                .split(";")
                .map(Game::parse)
                .collect::<Vec<_>>();

            return if games.iter().all(|g| g.check()) {
                game_id
            } else {
                0
            };
        })
        .sum();

    return result;
}

pub fn process(input: String) {
    let result = part2(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
 Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
 Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
 Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
 Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 2727);
    }

    #[test]
    fn part2_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 56580);
    }
}
