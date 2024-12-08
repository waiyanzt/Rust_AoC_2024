use regex::Regex;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input.txt")?;

    let instruction_re = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))")?;
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut total: i64 = 0;
    let mut multiplications_enabled = true;

    for cap in instruction_re.find_iter(&input) {
        let matched_text = cap.as_str();

        match matched_text {
            "do()" => multiplications_enabled = true,
            "don't()" => multiplications_enabled = false,
            _ => {
                if multiplications_enabled {
                    if let Some(num_cap) = mul_re.captures(matched_text) {
                        let x: i64 = num_cap[1].parse().unwrap();
                        let y: i64 = num_cap[2].parse().unwrap();
                        total += x * y;
                    }
                }
            }
        }
    }

    println!("The result for pt2 is {}", total);
    Ok(())
}
