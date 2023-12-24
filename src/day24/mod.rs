use glam::{dvec2, dvec3, DVec2, DVec3, Vec2};
use itertools::{iproduct, Itertools};
use z3::ast::Ast;

#[derive(Debug)]
struct Hailstone {
    position: DVec3,
    trajectory: DVec3,
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
            position: DVec3::new(p[0], p[1], p[2]),
            trajectory: DVec3::new(t[0], t[1], t[2]),
        }
    }

    fn intersection_2d(&self, other: &Hailstone) -> Option<DVec2> {
        // https://www.wikihow.com/Find-the-Equation-of-a-Line
        // https://www.wikihow.com/Algebraically-Find-the-Intersection-of-Two-Lines

        // y = m*x + b

        // Calculate Slope:
        // m = (y2-y1)/(x2-x1)
        // m = (p.y + t.y - p.y) / (p.x + t.x - p.x)
        // m = t.y/t.x

        let m1 = self.trajectory.y / self.trajectory.x;
        let m2 = other.trajectory.y / other.trajectory.x;

        // Calculate b (y-intercept):
        // p.y = m*p.x + b
        // b = p.y - m*p.x

        let b1 = self.position.y - m1 * self.position.x;
        let b2 = other.position.y - m2 * other.position.x;

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

    fn intersection_3d(&self, other: &Hailstone) -> Option<DVec3> {
        let p = self.position; // P1
        let v = self.trajectory; // v
        let q = other.position; // Q1
        let u = other.trajectory; // u

        // find a = v x u
        let a = v.cross(u); // cross product

        // find dot product = (v x u) . (v x u)
        let dot = a.dot(a); // inner product

        // if v and u are parallel (v x u = 0), then no intersection, return NaN point
        if dot == 0. {
            return None;
        }

        // find b = (Q1-P1) x u
        let b = (q - p).cross(u); // cross product

        // find t = (b.a)/(a.a) = ((Q1-P1) x u) .(v x u) / (v x u) . (v x u)
        let t = b.dot(a) / dot;

        // find intersection point by substituting t to the line1 eq
        let point = p + (t * v);

        return Some(point);
    }

    fn along_trajectory(&self, other: &DVec2) -> bool {
        if self.trajectory.x > 0. {
            return other.x > self.position.x;
        } else if self.trajectory.x < 0. {
            return other.x < self.position.x;
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

pub fn part2_bf(input: &str) -> usize {
    let hail = parse(input);

    let result = iproduct!(0.., 0.., 0.., 0.., 0.., 0..)
        .find (|(px, py, pz, tx, ty, tz)| {
        let this = Hailstone {
            position: dvec3(*px as f64, *py as f64, *pz as f64),
            trajectory: dvec3(*tx as f64, *ty as f64, *tz as f64),
        };
        let fits = hail.iter().take(3).all(|other| {
            if let Some(intersection) = this.intersection_3d(&other) {
                let t1 = this.position.distance(intersection) / this.trajectory;
                let t2 = other.position.distance(intersection) / other.trajectory;

                return t1 == t2;
            } else {
                return false;
            }
        });
        return fits;
    });

    dbg!(&result);

    return 0;
}

pub fn part2(input: &str) -> usize {
    let hail = parse(input);

    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, hs) in hail.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, hs.position.x);
        let va = z3::ast::Int::from_i64(&context, hs.trajectory.x);
        let b = z3::ast::Int::from_i64(&context, hs.position.y);
        let vb = z3::ast::Int::from_i64(&context, hs.trajectory.y);
        let c = z3::ast::Int::from_i64(&context, hs.position.z);
        let vc = z3::ast::Int::from_i64(&context, hs.trajectory.z);

        let t_i = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t_i.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t_i.clone())._eq(&(a + va * t_i.clone())));
        solver.assert(&(y.clone() + vy.clone() * t_i.clone())._eq(&(b + vb * t_i.clone())));
        solver.assert(&(z.clone() + vz.clone() * t_i.clone())._eq(&(c + vc * t_i.clone())));
    }
    if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            println!("Failed to solve!");
            return;
        };
        println!("{}", m.eval(&(x + y + z), true).unwrap());
    }

    return 0;
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

    // #[test]
    fn part2_example() {
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
