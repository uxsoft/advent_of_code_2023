use std::{env, fs};

mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "src\\day2\\sharp.txt".to_string();
    let filename = args.get(1).unwrap_or(&default_filename);

    let input = fs::read_to_string(filename).expect("Wrong file location");

    day2::process(input);
}
