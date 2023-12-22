use glam::IVec3;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Brick {
    id: usize,
    from: IVec3,
    to: IVec3,
}

impl Brick {
    fn parse(id: usize, input: &str) -> Brick {
        let (from_str, to_str) = input.split_once('~').unwrap();

        let f = from_str
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect_vec();
        let t = to_str
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect_vec();

        Brick {
            id,
            from: IVec3::new(f[0], f[1], f[2]),
            to: IVec3::new(t[0], t[1], t[2]),
        }
    }

    fn intersects(&self, other: &Brick) -> bool {
        let intersect_x = !(other.from.x > self.to.x || self.from.x > other.to.x);
        let intersect_y = !(other.from.y > self.to.y || self.from.y > other.to.y);

        return intersect_x && intersect_y;
    }

    fn height(&self) -> i32 {
        self.to.z - self.from.z + 1
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| Brick::parse(i, l))
        .collect()
}

struct Graph {
    bricks: Vec<Brick>,
    layers: HashMap<usize, usize>,
    above: HashMap<usize, Vec<usize>>,
    below: HashMap<usize, Vec<usize>>,
}
impl Graph {
    fn new(bricks: Vec<Brick>) -> Graph {
        let bricks = bricks.into_iter().sorted_by_key(|b| b.from.z).collect_vec();

        let mut layers: HashMap<usize, usize> = HashMap::new();
        let mut above: HashMap<usize, Vec<usize>> = bricks.iter().map(|b| (b.id, vec![])).collect();
        let mut below: HashMap<usize, Vec<usize>> = bricks.iter().map(|b| (b.id, vec![])).collect();

        for i in 0..bricks.len() {
            let brick = &bricks[i];
            let intersections = bricks[..i]
                .iter()
                .filter(|b| brick.intersects(b))
                .map(|b| b)
                .collect_vec();

            let layer = intersections
                .iter()
                .map(|b| layers.get(&b.id).map(|l| *l).unwrap() + b.height() as usize)
                .max()
                .unwrap_or(1);

            layers.insert(brick.id, layer);

            let below_this = intersections
                .iter()
                .filter(|b| {
                    let b_layer = *layers.get(&b.id).unwrap();
                    let b_height = b.height() as usize;
                    layer == (b_layer + b_height)
                })
                .collect_vec();

            for b in below_this.iter() {
                below
                    .entry(brick.id)
                    .and_modify(|v| v.push(b.id))
                    .or_insert(vec![b.id]);

                above
                    .entry(b.id)
                    .and_modify(|v| v.push(brick.id))
                    .or_insert(vec![brick.id]);
            }
        }

        Graph {
            bricks,
            layers,
            above,
            below,
        }
    }

    fn is_brick_stable(&self, brick: &Brick) -> bool {
        let empty_vec = vec![];
        let bricks_on_top = self.above.get(&brick.id).unwrap_or(&empty_vec);
        let is_stable = bricks_on_top
            .iter()
            .all(|bot| self.below.get(bot).unwrap_or(&empty_vec).len() > 1);

        return is_stable;
    }

    fn dependant_bricks(&self, brick: &Brick) -> usize {
        let empty_vec = vec![];

        let mut removed = HashSet::new();
        removed.insert(brick.id);

        let mut queue = VecDeque::new();
        for b_id in self
            .above
            .get(&brick.id)
            .unwrap_or(&empty_vec)
            .iter()
            .sorted_by_key(|b_id| self.layers[b_id])
        {
            queue.push_back(b_id);
        }

        while let Some(b_id) = queue.pop_front() {
            let above_b = self.above.get(b_id).unwrap_or(&empty_vec);
            let below_b = self.below.get(b_id).unwrap_or(&empty_vec);

            if below_b.iter().all(|bb_id| removed.contains(bb_id)) {
                // If all below are removed, this node also falls
                removed.insert(*b_id);

                for ab_id in above_b.iter().sorted_by_key(|b_id| self.layers[b_id]) {
                    queue.push_back(ab_id);
                }
            }
        }

        return removed.len()-1;
    }
}

pub fn part1(input: &str) -> usize {
    let incoming_bricks = parse(input);
    let graph = Graph::new(incoming_bricks);

    let stable_bricks = graph
        .bricks
        .iter()
        .filter(|brick| graph.is_brick_stable(brick))
        .collect_vec();

    return stable_bricks.len();
}

pub fn part2(input: &str) -> usize {
    let incoming_bricks = parse(input);
    let graph = Graph::new(incoming_bricks);

    let result = graph
        .bricks
        .iter()
        .map(|brick| graph.dependant_bricks(brick))
        .sum::<usize>();

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

    const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 5);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 457);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 79122);
    }
}
