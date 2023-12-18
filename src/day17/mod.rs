use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

fn parse(input: &str) -> HashMap<IVec2, u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect()
}

fn find_shortest_path(grid: &HashMap<IVec2, u32>) -> u32 {
    let max_x = grid.keys().map(|i| i.x).max().unwrap();
    let max_y = grid.keys().map(|i| i.y).max().unwrap();

    // let boundaries = IVec2::new(column_count, row_count);
    let goal = IVec2::new(max_x, max_y);
    let (path, distance) = dijkstra(
        &(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |(position, deque)| {
            [
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
                IVec2::new(0, -1),
                IVec2::new(0, 1),
            ]
            .into_iter()
            .filter_map(|pos_diff| {
                let next_position = pos_diff + *position;
                if grid.contains_key(&next_position) {
                    if deque.len() > 2 && deque[1] == next_position {
                        return None;
                    }

                    let mut new_deque = deque.clone();
                    new_deque.push_front(next_position);
                    if new_deque.len() == 5 {
                        let dir = new_deque[1] - new_deque[0];
                        let a = new_deque[2] - new_deque[1];
                        let b = new_deque[3] - new_deque[2];
                        let c = new_deque[4] - new_deque[3];
                        // if we've moved in the same direction 4 times
                        let three_forward_check = [a, b, c].iter().all(|a_dir| a_dir == &dir);

                        if three_forward_check {
                            None
                        } else {
                            new_deque.pop_back();
                            Some((next_position, new_deque))
                        }
                    } else {
                        Some((next_position, new_deque))
                    }
                } else {
                    None
                }
            })
            .map(|pos| {
                let next_cost = *grid.get(&pos.0).unwrap();
                (pos, next_cost)
            })
            .collect::<Vec<((IVec2, VecDeque<IVec2>), u32)>>()
        },
        |(win, _deque)| {
            // todo: Not too far in a straight
            win == &goal
        },
    )
    .expect("should have a valid path");

    return distance;
}

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);

    let d = find_shortest_path(&grid);

    println!("{d:?}");

    // let distance_to_bottom_right = d
    //     .get(&(grid[0].len() - 1, grid.len() - 1))
    //     .expect("Expected a distance to bottom right");

    return d;
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

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 102);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 843);
    }

    // #[test]
    fn part2_example() {
        let input = "";
        let result = part2(EXAMPLE);
        assert_eq!(result, 0);
    }

    // #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
