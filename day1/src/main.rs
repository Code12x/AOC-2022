use std::fs;
use std::io;

fn main() {
    let input = getInput().expect("Error opening file!");
}

fn getInput() -> Result<String, io::Error> {
    fs::read_to_sting("puzzle_input.txt");
}
