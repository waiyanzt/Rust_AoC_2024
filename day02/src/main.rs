// main.rs
use day2::is_safe_with_dampener;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_count = reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();
    println!("Number of safe reports with dampener: {}", safe_count);
}
