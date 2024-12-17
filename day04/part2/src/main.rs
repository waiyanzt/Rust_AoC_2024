use std::fs;

#[derive(Clone, Copy)]
enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count = count_xmas_patterns(&grid);
    println!("Found {} X-MAS patterns", count);
}

fn count_xmas_patterns(grid: &[Vec<char>]) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    // Look for center 'A's
    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == 'A' {
                // For each 'A', check if it forms a valid X pattern
                if is_valid_x_pattern(grid, i, j, rows, cols) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_valid_x_pattern(grid: &[Vec<char>], row: i32, col: i32, rows: i32, cols: i32) -> bool {
    // Check if either diagonal can be "MAS" or "SAM"
    let upper_left_to_lower_right = check_diagonal(
        grid,
        row,
        col,
        Direction::UpLeft,
        Direction::DownRight,
        rows,
        cols,
    );

    let upper_right_to_lower_left = check_diagonal(
        grid,
        row,
        col,
        Direction::UpRight,
        Direction::DownLeft,
        rows,
        cols,
    );

    upper_left_to_lower_right && upper_right_to_lower_left
}

fn check_diagonal(
    grid: &[Vec<char>],
    row: i32,
    col: i32,
    dir1: Direction,
    dir2: Direction,
    rows: i32,
    cols: i32,
) -> bool {
    // Check both forward and backward for this diagonal
    (check_mas_direction(grid, row, col, dir1, dir2, rows, cols, &['M', 'A', 'S'])
        || check_mas_direction(grid, row, col, dir1, dir2, rows, cols, &['S', 'A', 'M']))
}

fn check_mas_direction(
    grid: &[Vec<char>],
    row: i32,
    col: i32,
    dir1: Direction,
    dir2: Direction,
    rows: i32,
    cols: i32,
    pattern: &[char],
) -> bool {
    // Check first half of diagonal
    let (dr1, dc1) = dir1.get_offset();
    let pos1_row = row + dr1;
    let pos1_col = col + dc1;

    // Check second half of diagonal
    let (dr2, dc2) = dir2.get_offset();
    let pos2_row = row + dr2;
    let pos2_col = col + dc2;

    if !is_valid_position(pos1_row, pos1_col, rows, cols)
        || !is_valid_position(pos2_row, pos2_col, rows, cols)
    {
        return false;
    }

    grid[pos1_row as usize][pos1_col as usize] == pattern[0]
        && grid[pos2_row as usize][pos2_col as usize] == pattern[2]
}

fn is_valid_position(row: i32, col: i32, rows: i32, cols: i32) -> bool {
    row >= 0 && row < rows && col >= 0 && col < cols
}
