fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[path = "../src/day1/mod.rs"]
mod day1;

#[divan::bench]
fn day1() {
    day1::part2(divan::black_box(include_str!("../src/day1/input.txt",)));
}

#[path = "../src/day2/mod.rs"]
mod day2;

#[divan::bench]
fn day2() {
    day2::part2(divan::black_box(include_str!("../src/day2/input.txt",)));
}

#[path = "../src/day3/mod.rs"]
mod day3;

#[divan::bench]
fn day3() {
    day3::part2(divan::black_box(include_str!("../src/day3/input.txt",)));
}

#[path = "../src/day4/mod.rs"]
mod day4;

#[divan::bench]
fn day4() {
    day4::part2(divan::black_box(include_str!("../src/day4/input.txt",)));
}

#[path = "../src/day5/mod.rs"]
mod day5;

#[divan::bench]
fn day5() {
    day5::part2(divan::black_box(include_str!("../src/day5/input.txt",)));
}

#[path = "../src/day6/mod.rs"]
mod day6;

#[divan::bench]
fn day6() {
    day6::part2(divan::black_box(include_str!("../src/day6/input.txt",)));
}

#[path = "../src/day7/mod.rs"]
mod day7;

#[divan::bench]
fn day7() {
    day7::part2(divan::black_box(include_str!("../src/day7/input.txt",)));
}

#[path = "../src/day8/mod.rs"]
mod day8;

#[divan::bench]
fn day8() {
    day8::part2(divan::black_box(include_str!("../src/day8/input.txt",)));
}

#[path = "../src/day9/mod.rs"]
mod day9;

#[divan::bench]
fn day9() {
    day9::part2(divan::black_box(include_str!("../src/day9/input.txt",)));
}

#[path = "../src/day10/mod.rs"]
mod day10;

#[divan::bench]
fn day10() {
    day10::part2(divan::black_box(include_str!("../src/day10/input.txt",)));
}

#[path = "../src/day11/mod.rs"]
mod day11;

#[divan::bench]
fn day11() {
    day11::part2(
        divan::black_box(include_str!("../src/day11/input.txt",)),
        1_000_000,
    );
}

#[path = "../src/day12/mod.rs"]
mod day12;

#[divan::bench]
fn day12() {
    day12::part2(divan::black_box(include_str!("../src/day12/input.txt",)));
}

#[path = "../src/day13/mod.rs"]
mod day13;

#[divan::bench]
fn day13() {
    day13::part2(divan::black_box(include_str!("../src/day13/input.txt",)));
}

#[path = "../src/day14/mod.rs"]
mod day14;

#[divan::bench]
fn day14() {
    day14::part2(divan::black_box(include_str!("../src/day14/input.txt",)));
}