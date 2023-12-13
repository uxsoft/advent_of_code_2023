use std::{env, fs, path::Path};

mod day13;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let default_filename = Path::new("src/day13/input.txt");
    // let filename = args.get(1).unwrap_or(&default_filename);

    let input = fs::read_to_string(default_filename).expect("Wrong file location");

    day13::process(input);
}