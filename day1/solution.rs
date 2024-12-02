use std::fs::read_to_string;

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Read the file line by line!
    if let Ok(contents) = read_to_string("input.txt") {
        for line in contents.lines() {
            let numbers: Vec<&str> = line.split_whitespace().collect();

            if numbers.len() == 2 {
                if let Ok(left_num) = numbers[0].parse::<i32>() {
                    left.push(left_num);
                }
                if let Ok(right_num) = numbers[1].parse::<i32>() {
                    right.push(right_num);
                }
            }
        }
    }

    left.sort();
    right.sort();

    let total: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total sum of differences: {}", total);
}
