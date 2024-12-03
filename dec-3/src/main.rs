use std::env;
use std::fs;
use regex::Regex;


fn sum_of_multiplications(contents: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for cap in re.captures_iter(&contents) {
        let num1 = &cap[1];
        let num2 = &cap[2];
        total += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
    }
    return total;
}

fn main() {
    let file_path: String = env::args()
    .nth(1)
    .expect("Please provide a file path");

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    // Part 1

    println!("Part 1: {}", sum_of_multiplications(&contents));

    // Part 2
    
    // List of start indices
    let mut start_indices: Vec<_> = [0].to_vec();
    let start_regex = Regex::new(r"do\(\)").unwrap();
    for cap in start_regex.captures_iter(&contents) {
        let whole_match = cap.get(0).unwrap();
        let start_index = whole_match.start();
        start_indices.push(start_index);
    }
    println!("Start indices: {:?}", start_indices);
    
    // List of end indices
    let end_regex = Regex::new(r"don\'t\(\)").unwrap();
    let mut end_indices: Vec<_> = [].to_vec();
    for cap in end_regex.captures_iter(&contents) {
        let whole_match = cap.get(0).unwrap();
        let end_index = whole_match.end();
        end_indices.push(end_index);
    }
    println!("End indices: {:?}", end_indices);

    let mut total = 0;
    let mut index_of_current_end = 0;
    let mut index_of_current_start = 0;
    while index_of_current_start < start_indices.len() {
        let mut current_end = end_indices[index_of_current_end];
        let mut current_start = start_indices[index_of_current_start];

        // Move end to after the start
        while current_end < current_start && index_of_current_end < (end_indices.len() -1) {
            index_of_current_end += 1;
            current_end = end_indices[index_of_current_end];
        }
        // Handle case where there are no more end indices
        if current_end < current_start {
            println!("No more end indices");
            println!("Adding to end from: {}", current_start);
            let current_contents = &contents[current_start..contents.len()];
            total += sum_of_multiplications(&current_contents);
            break;
        }

        // Add total
        println!("Adding from: {} to {}", current_start, current_end);
        let current_contents = &contents[current_start..current_end];
        total += sum_of_multiplications(&current_contents);

        // Make sure the start is after the end before looping again
        while current_start < current_end && index_of_current_start < (start_indices.len() -1) {
            index_of_current_start += 1;
            current_start = start_indices[index_of_current_start];
        }
        if current_start < current_end {
            println!("No more start indices");
            break;
        }
    }
    println!("Part 2: {}", total);


}