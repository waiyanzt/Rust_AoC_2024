mod grid;
use grid::Grid;
use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // right
    (1, 0),   // down
    (0, -1),  // left
    (-1, 0),  // up
    (1, 1),   // down-right
    (-1, -1), // up-left
    (1, -1),  // down-left
    (-1, 1),  // up-right
];

fn main() -> std::io::Result<()> {
    // Read and parse input
    let contents = fs::read_to_string("input.txt")?;
    let grid_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let grid = Grid::new(grid_data);
    let mut count = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            for &(dir_row, dir_col) in DIRECTIONS.iter() {
                if grid.check_xmas_at_position(row as i32, col as i32, dir_row, dir_col) {
                    count += 1;
                }
            }
        }
    }

    println!("Found {} XMAS occurrences", count);
    Ok(())
}
