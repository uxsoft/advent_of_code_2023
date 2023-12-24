use glam::I64Vec3;

struct Hailstone {
    position: I64Vec3,
    trajectory: I64Vec3,
}

impl Hailstone {
    fn parse(input: &str) -> Hailstone {
        let (pos_str, traj_str) = input.split_once(" @ ").unwrap();
        let position = pos_str
            .split(", ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        
        let trajectory = traj_str
            .split(", ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Hailstone {
            position,
            trajectory,
        }
    }
}

pub fn part1(input: &str) -> usize {
    return 0;
}

pub fn part2(input: &str) -> usize {
    return 0;
}

pub fn process(input: String) {
    use std::time::Instant;
    let now = Instant::now();
    let result = part1(&input);
    println!("Result: {result}");
    println!("Finished in: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
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
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
