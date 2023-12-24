use glam::{dvec2, dvec3, DVec2, DVec3, Vec2, I64Vec3};
use itertools::{iproduct, Itertools};
use z3::{Config, Context, Solver};
use z3::ast::{Ast, Int};

#[derive(Debug)]
struct Hailstone {
    position: I64Vec3,
    velocity: I64Vec3,
}

impl Hailstone {
    fn parse(input: &str) -> Hailstone {
        let (pos_str, traj_str) = input.split_once(" @ ").unwrap();
        let p = pos_str
            .split(", ")
            .map(str::trim)
            .map(str::parse)
            .map(Result::unwrap)
            .collect_vec();

        let t = traj_str
            .split(", ")
            .map(str::trim)
            .map(str::parse)
            .map(Result::unwrap)
            .collect_vec();

        Hailstone {
            position: I64Vec3::new(p[0], p[1], p[2]),
            velocity: I64Vec3::new(t[0], t[1], t[2]),
        }
    }

    fn intersection_2d(&self, other: &Hailstone) -> Option<DVec2> {
        // https://www.wikihow.com/Find-the-Equation-of-a-Line
        // https://www.wikihow.com/Algebraically-Find-the-Intersection-of-Two-Lines

        // y = m*x + b

        // Calculate Slope:
        // m = (y2-y1)/(x2-x1)
        // m = (p.y + v.y - p.y) / (p.x + v.x - p.x)
        // m = v.y/v.x

        let m1 = self.velocity.y as f64 / self.velocity.x as f64;
        let m2 = other.velocity.y as f64 / other.velocity.x as f64;

        // Calculate b (y-intercept):
        // p.y = m*p.x + b
        // b = p.y - m*p.x

        let b1 = self.position.y as f64 - m1 * self.position.x as f64;
        let b2 = other.position.y as f64 - m2 * other.position.x as f64;

        // intersection, put the two equations against each other (at which x are the y equal):
        // y = m1*x + b1, y = m2*x + b2
        // m1*x + b1 = m2*x + b2
        // x = (b2 - b1) / (m1 - m2)
        // if m2-m1 == 0 then lines have no intersection

        if m1 == m2 {
            return None;
        }

        let x = (b2 - b1) / (m1 - m2);
        let y = m1 * x + b1;

        Some(dvec2(x, y))
    }

    fn along_trajectory(&self, other: &DVec2) -> bool {
        if self.velocity.x > 0 {
            return other.x > self.position.x as f64;
        } else if self.velocity.x < 0 {
            return other.x < self.position.x as f64;
        }

        return false;
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input.lines().map(Hailstone::parse).collect_vec()
}

pub fn part1(input: &str, from: i64, to: i64) -> usize {
    let hail = parse(input);

    let intersections = hail
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.intersection_2d(b)))
        .filter_map(|(a, b, io)| io.map(|i| (a, b, i))) // Only those who crossed
        .filter(|(_, _, i)| {
            // Only those who crossed in the area
            i.x >= from as f64 && i.x <= to as f64 && i.y >= from as f64 && i.y <= to as f64
        })
        .filter(|(a, b, i)| a.along_trajectory(i) && b.along_trajectory(i)) // Only those who crossed in the direction of travel
        .collect_vec();

    return intersections.len();
}

pub fn part2(input: &str) -> usize {
    let hail = parse(input);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hail {
        let pxn = Int::from_i64(&ctx, hailstone.position.x);
        let pyn = Int::from_i64(&ctx, hailstone.position.y);
        let pzn = Int::from_i64(&ctx, hailstone.position.z);
        let vxn = Int::from_i64(&ctx, hailstone.velocity.x);
        let vyn = Int::from_i64(&ctx, hailstone.velocity.y);
        let vzn = Int::from_i64(&ctx, hailstone.velocity.z);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    return (x + y + z).try_into().unwrap();
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

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_example() {
        let result = part1(EXAMPLE, 7, 27);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input, 200000000000000, 400000000000000);
        // 11942 is too low
        assert_eq!(result, 16502);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 47);
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 673641951253289);
    }
}
