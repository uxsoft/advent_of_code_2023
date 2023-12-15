use std::{env, fs, path::Path};

pub mod day15;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let default_filename = Path::new("src/day15/input.txt");
    // let filename = args.get(1).unwrap_or(&default_filename);

    let input = fs::read_to_string(default_filename).expect("Wrong file location");

    day15::process(input);
}