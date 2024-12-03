use std::env;
use std::fs;
use std::collections::HashMap;
fn main() {
    // Part 1
    let file_path: String = env::args()
        .nth(1)
        .expect("Please provide a file path");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut column_a_list: Vec<i32> = Vec::new();
    let mut column_b_list: Vec<i32> = Vec::new();

    for line in lines {
        let column_a_value: &str = line.split_whitespace().nth(0).expect("Could not get column A value");
        let column_b_value: &str = line.split_whitespace().nth(1).expect("Could not get column B value");
        column_a_list.push(column_a_value.parse().expect("Could not parse column A value"));
        column_b_list.push(column_b_value.parse().expect("Could not parse column B value"));
    }

    column_a_list.sort();
    column_b_list.sort();

    let mut diff_sum: i32 = 0;

    for i in 0..column_a_list.len() {
        diff_sum += (column_b_list[i] - column_a_list[i]).abs();
    }

    println!("Diff sum: {}", diff_sum);


    // Part 2
    let mut similarity_score: i32 = 0;

    // Store the frequency of each value in column B
    let mut column_b_frequency: HashMap<i32, i32> = HashMap::new();
    for value in column_b_list {
        let count = column_b_frequency.entry(value).or_insert(0);
        *count += 1;
    }

    for value in column_a_list {
        let count: &mut i32 = column_b_frequency.entry(value).or_insert(0);
        if *count > 0 {
            similarity_score += *count * value;
        }
    }

    println!("Similarity score: {:?}", similarity_score);


}