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

pub fn process(input: String) {
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

            let max_game = games
                .iter()
                .fold(Game::default(), |a, b| Game {
                    red: a.red.max(b.red),
                    blue: a.blue.max(b.blue),
                    green: a.green.max(b.green),
                });

            return max_game.red * max_game.green * max_game.blue;
        })
        .sum();

    println!("Result: {}", result);
}

pub fn part1(input: String) {
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

    println!("Result: {}", result);
}
