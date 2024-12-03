use std::env;
use std::fs;

fn are_numbers_safe(current_number: i32, next_number: i32, prev_gap_incrementing:bool)->bool {
    // Same number case
    if current_number == next_number {
        return false;
    }

    // Changes in incrementing or decrementing case
    if prev_gap_incrementing != (current_number < next_number) {
        return false;
    }

    // Check for gaps greater than 3
    if (current_number - next_number).abs() > 3 {
        return false;
    }
    return true;
}

fn is_line_safe(line: &str, use_dampener: bool)->bool {
    // Input is a string of numbers separated by spaces
    // We need to convert this to a vector of numbers
    let numbers: Vec<i32> = line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut is_safe:bool = true;
    let mut possible_mutations: Vec<Vec<i32>> = Vec::new();
    possible_mutations.push(numbers.clone());
    if use_dampener {
        // Check the line but with each number removed
        for index in 0..numbers.len() {
            let mut new_numbers = numbers.clone();
            new_numbers.remove(index);
            possible_mutations.push(new_numbers);
        }
    }
    let mut is_any_safe = false;
    for mutation in possible_mutations {
        is_safe = true;
        let mut prev_gap_incrementing:bool = mutation[0] < mutation[1];
        for index in 0..mutation.len()-1 {
            let current_number = mutation[index];
            let next_number = mutation[index + 1];
            is_safe = are_numbers_safe(current_number, next_number, prev_gap_incrementing);
            if !is_safe {
                break;
            }
            prev_gap_incrementing = current_number < next_number;
        }
        if is_safe {
            is_any_safe = true;
        }
    }

    return is_any_safe;
}

fn main() {
    // Part 1
    let file_path: String = env::args()
        .nth(1)
        .expect("Please provide a file path");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut safe_lines = 0;
    for line in lines {
        if is_line_safe(line, false) {
            safe_lines += 1;
        }
    }

    println!("Part 1: {}", safe_lines);

    // Part 2
    let lines: Vec<&str> = contents.lines().collect();
    let mut safe_lines = 0;
    for line in lines {
        if is_line_safe(line, true) {
            safe_lines += 1;
        }
    }

    println!("Part 2: {}", safe_lines);
}
