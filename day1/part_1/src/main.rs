use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_input.txt").expect("Error opening the file");

    let mut calories_per_elf: Vec<i32> = Vec::new();

    let mut previous_was_new_line = false;
    let mut current_elfs_calories = 0;
    for line in input.lines() {
        if previous_was_new_line && line == "\n".to_owned() {
            calories_per_elf.push(current_elfs_calories);
            current_elfs_calories = 0;
            previous_was_new_line = true;
        }else {
            current_elfs_calories += ;

        }
    }
}

