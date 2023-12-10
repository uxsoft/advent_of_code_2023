use std::{env, fs, path::Path};

mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = Path::new("src/day7/sharp.txt");
    // let filename = args.get(1).unwrap_or(&default_filename);

    let input = fs::read_to_string(default_filename).expect("Wrong file location");

    day7::process(input);
}