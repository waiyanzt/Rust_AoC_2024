use regex::Regex;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input.txt")?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut total: i64 = 0;
    for cap in re.captures_iter(&input) {
        let x: i64 = cap[1].parse().unwrap();
        let y: i64 = cap[2].parse().unwrap();
        let result = x * y;
        total += result;
    }

    println!("The result for pt1 is {}", total);
    Ok(())
}
