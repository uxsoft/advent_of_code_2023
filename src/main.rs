use std::{env, fs};

mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "src\\day5\\sharp.txt".to_string();
    let filename = args.get(1).unwrap_or(&default_filename);

    let input = fs::read_to_string(filename).expect("Wrong file location");

    day5::process(input);
}
