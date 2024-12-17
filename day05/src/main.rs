pub mod part1;
pub mod part2;

use part1::part1;
use part2::part2;

fn main() {
    use std::time::Instant;
    println!("Day 5");
    println!("---------------");
    let now = Instant::now();
    let solution1 = part1();
    println!("part1: {},", solution1);
    let elapsed = now.elapsed();
    println!("part1 : {:.2?}", elapsed);

    let now = Instant::now();
    let solution2 = part2();
    println!("part2: {},", solution2);
    let elapsed = now.elapsed();
    println!("part2 : {:.2?}", elapsed);
    println!("---------------");
}
