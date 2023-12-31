fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[path = "../src/day1/mod.rs"]
mod day1;

#[divan::bench(max_time = 1)]
fn day1() {
    day1::part2(divan::black_box(include_str!("../src/day1/input.txt")));
}

#[path = "../src/day2/mod.rs"]
mod day2;

#[divan::bench(max_time = 1)]
fn day2() {
    day2::part2(divan::black_box(include_str!("../src/day2/input.txt")));
}

#[path = "../src/day3/mod.rs"]
mod day3;

#[divan::bench(max_time = 1)]
fn day3() {
    day3::part2(divan::black_box(include_str!("../src/day3/input.txt")));
}

#[path = "../src/day4/mod.rs"]
mod day4;

#[divan::bench(max_time = 1)]
fn day4() {
    day4::part2(divan::black_box(include_str!("../src/day4/input.txt")));
}

#[path = "../src/day5/mod.rs"]
mod day5;

#[divan::bench(max_time = 1)]
fn day5() {
    day5::part2(divan::black_box(include_str!("../src/day5/input.txt")));
}

#[path = "../src/day6/mod.rs"]
mod day6;

#[divan::bench(max_time = 1)]
fn day6() {
    day6::part2(divan::black_box(include_str!("../src/day6/input.txt")));
}

#[path = "../src/day7/mod.rs"]
mod day7;

#[divan::bench(max_time = 1)]
fn day7() {
    day7::part2(divan::black_box(include_str!("../src/day7/input.txt")));
}

#[path = "../src/day8/mod.rs"]
mod day8;

#[divan::bench(max_time = 1)]
fn day8() {
    day8::part2(divan::black_box(include_str!("../src/day8/input.txt")));
}

#[path = "../src/day9/mod.rs"]
mod day9;

#[divan::bench(max_time = 1)]
fn day9() {
    day9::part2(divan::black_box(include_str!("../src/day9/input.txt")));
}

#[path = "../src/day10/mod.rs"]
mod day10;

#[divan::bench(max_time = 1)]
fn day10() {
    day10::part2(divan::black_box(include_str!("../src/day10/input.txt")));
}

#[path = "../src/day11/mod.rs"]
mod day11;

#[divan::bench(max_time = 1)]
fn day11() {
    day11::part2(
        divan::black_box(include_str!("../src/day11/input.txt")),
        1_000_000,
    );
}

#[path = "../src/day12/mod.rs"]
mod day12;

#[divan::bench(max_time = 1)]
fn day12() {
    day12::part2(divan::black_box(include_str!("../src/day12/input.txt")));
}

#[path = "../src/day13/mod.rs"]
mod day13;

#[divan::bench(max_time = 1)]
fn day13() {
    day13::part2(divan::black_box(include_str!("../src/day13/input.txt")));
}

#[path = "../src/day14/mod.rs"]
mod day14;

#[divan::bench(max_time = 1)]
fn day14() {
    day14::part2(divan::black_box(include_str!("../src/day14/input.txt")));
}

#[path = "../src/day15/mod.rs"]
mod day15;

#[divan::bench(max_time = 1)]
fn day15() {
    day15::part2(divan::black_box(include_str!("../src/day15/input.txt")));
}

#[path = "../src/day16/mod.rs"]
mod day16;

#[divan::bench(max_time = 1)]
fn day16() {
    day16::part2(divan::black_box(include_str!("../src/day16/input.txt")));
}

#[path = "../src/day17/mod.rs"]
mod day17;

#[divan::bench(max_time = 1)]
fn day17() {
    day17::part2(divan::black_box(include_str!("../src/day17/input.txt")));
}

#[path = "../src/day18/mod.rs"]
mod day18;

#[divan::bench(max_time = 1)]
fn day18() {
    day18::part2(divan::black_box(include_str!("../src/day18/input.txt")));
}

#[path = "../src/day19/mod.rs"]
mod day19;

#[divan::bench(max_time = 1)]
fn day19() {
    day19::part2(divan::black_box(include_str!("../src/day19/input.txt")));
}

#[path = "../src/day20/mod.rs"]
mod day20;

#[divan::bench(max_time = 1)]
fn day20() {
    day20::part2(divan::black_box(include_str!("../src/day20/input.txt")));
}

#[path = "../src/day21/mod.rs"]
mod day21;

#[divan::bench(max_time = 1)]
fn day21() {
    day21::part2(
        divan::black_box(include_str!("../src/day21/input.txt")),
        26501365,
    );
}

#[path = "../src/day22/mod.rs"]
mod day22;

#[divan::bench(max_time = 1)]
fn day22() {
    day22::part2(divan::black_box(include_str!("../src/day22/input.txt")));
}

#[path = "../src/day23/mod.rs"]
mod day23;

#[divan::bench(max_time = 1)]
fn day23() {
    day23::part2(divan::black_box(include_str!("../src/day23/input.txt")));
}

#[path = "../src/day24/mod.rs"]
mod day24;

#[divan::bench(max_time = 1)]
fn day24() {
    day24::part2(divan::black_box(include_str!("../src/day24/input.txt")));
}

#[path = "../src/day25/mod.rs"]
mod day25;

#[divan::bench(max_time = 1)]
fn day25() {
    day25::part1(divan::black_box(include_str!("../src/day25/input.txt")));
}
