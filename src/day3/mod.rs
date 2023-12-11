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
    pub fn parse(input: &str) -> Grid {
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

pub fn part1(input: &str) -> u32 {
    let grid = Grid::parse(input);

    let result: u32 = grid
        .numbers
        .iter()
        .filter(|n| n.has_symbol_neighbor(&grid))
        .map(|n| n.value)
        .sum();

    return result;
}

pub fn part2(input: &str) -> u32 {
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 557705);
    }

    #[test]
    fn part2_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part2(input);
        assert_eq!(result, 467835);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 84266818);
    }
}
