use std::{env, fs, path::Path};

mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = Path::new("src/day6/sharp.txt");
    // let filename = args.get(1).unwrap_or(&default_filename);

    let input = fs::read_to_string(default_filename).expect("Wrong file location");

    day6::process(input);
}
