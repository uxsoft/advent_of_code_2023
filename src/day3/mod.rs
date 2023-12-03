use std::collections::VecDeque;

struct Number {
    value: u32,
    x: usize,
    y: usize,
    length: usize,
}

struct Grid {
    grid: Vec<Vec<char>>,
    numbers: Vec<Number>,
    gears: Vec<(usize, usize)>,
}

impl Grid {
    pub fn parse(input: String) -> Grid {
        let grid = input
            .lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut numbers: Vec<Number> = Vec::new();
        let mut gears: Vec<(usize, usize)> = Vec::new();

        let mut value: u32 = 0;
        let mut length: usize = 0;

        fn terminate_number(
            value: &mut u32,
            numbers: &mut Vec<Number>,
            length: &mut usize,
            x: &usize,
            y: &usize,
        ) {
            if *value > 0 {
                numbers.push(Number {
                    value: *value,
                    x: *x - *length,
                    y: *y,
                    length: *length,
                });
                *value = 0;
                *length = 0;
            }
        }

        for (y, line) in grid.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                match char {
                    n if n.is_ascii_digit() => {
                        value = 10 * value + (*n as u32 - '0' as u32);
                        length += 1;
                    }
                    '*' => {
                        gears.push((x, y));
                        terminate_number(&mut value, &mut numbers, &mut length, &x, &y);
                    }
                    _ => {
                        terminate_number(&mut value, &mut numbers, &mut length, &x, &y);
                    }
                }
            }

            terminate_number(&mut value, &mut numbers, &mut length, &140, &y);
        }

        return Grid {
            grid,
            numbers,
            gears,
        };
    }
}

impl Number {
    pub fn has_symbol_neighbor(&self, grid: &Grid) -> bool {
        let mut coords: Vec<(i32, i32)> = Vec::new();

        // Coordinates to check
        coords.push((self.x as i32 - 1, self.y as i32));
        coords.push((self.x as i32 + self.length as i32, self.y as i32));

        for x in (self.x as i32 - 1)..=(self.x as i32 + self.length as i32) {
            coords.push((x, self.y as i32 - 1));
            coords.push((x, self.y as i32 + 1));
        }

        // Check all the adjacent coordinates
        for (x, y) in coords {
            if x >= 0 && y >= 0 {
                if let Some(line) = grid.grid.get(y as usize) {
                    if let Some(char) = line.get(x as usize) {
                        match char {
                            '.' => (),
                            x if x.is_ascii_digit() => (),
                            _ => return true,
                        }
                    }
                }
            }
        }

        return false;
    }

    pub fn is_neighbor_of(&self, x: &usize, y: &usize) -> bool {
        let mut coords: Vec<(i32, i32)> = Vec::new();

        // Coordinates to check
        coords.push((self.x as i32 - 1, self.y as i32));
        coords.push((self.x as i32 + self.length as i32, self.y as i32));

        for x in (self.x as i32 - 1)..=(self.x as i32 + self.length as i32) {
            coords.push((x, self.y as i32 - 1));
            coords.push((x, self.y as i32 + 1));
        }

        // Check all the adjacent coordinates
        for (cx, cy) in coords {
            if cx == *x as i32 && cy == *y as i32 {
                return true;
            }
        }

        return false;
    }
}

pub fn part1(input: String) {
    let grid = Grid::parse(input);

    let result: u32 = grid
        .numbers
        .iter()
        .filter(|n| n.has_symbol_neighbor(&grid))
        .map(|n| n.value)
        .sum();

    println!("Result: {}", result);
}

pub fn process(input: String) {
    let grid = Grid::parse(input);

    let result: u32 = grid
        .gears
        .iter()
        .map(|(g_x, g_y)| {
            grid.numbers
                .iter()
                .filter(|n| n.is_neighbor_of(g_x, g_y))
                .collect::<Vec<_>>()
        })
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().map(|p| p.value).product::<u32>())
        .sum();

    println!("Result: {}", result);
}
