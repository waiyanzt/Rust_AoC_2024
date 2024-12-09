pub struct Grid {
    pub data: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Grid { data, rows, cols }
    }

    pub fn is_valid_position(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32
    }

    pub fn get_char(&self, row: i32, col: i32) -> Option<char> {
        if self.is_valid_position(row, col) {
            Some(self.data[row as usize][col as usize])
        } else {
            None
        }
    }

    pub fn check_mas(&self, start_row: i32, start_col: i32, dir_row: i32, dir_col: i32) -> bool {
        let target = ['M', 'A', 'S'];

        // Check forward direction
        let mut forward = true;
        for i in 0..3 {
            let row = start_row + i * dir_row;
            let col = start_col + i * dir_col;

            match self.get_char(row, col) {
                Some(c) if c == target[i as usize] => continue,
                _ => {
                    forward = false;
                    break;
                }
            }
        }

        // Check backward direction
        let mut backward = true;
        for i in 0..3 {
            let row = start_row + i * dir_row;
            let col = start_col + i * dir_col;

            match self.get_char(row, col) {
                Some(c) if c == target[2 - i as usize] => continue,
                _ => {
                    backward = false;
                    break;
                }
            }
        }

        forward || backward
    }

    pub fn check_x_mas_at_position(&self, center_row: i32, center_col: i32) -> bool {
        // Check diagonal pairs: down-right + down-left and up-right + up-left
        let diagonal_pairs = [
            ((1, 1), (1, -1)),   // down-right + down-left
            ((-1, 1), (-1, -1)), // up-right + up-left
            ((1, 1), (-1, -1)),  // down-right + up-left
            ((1, -1), (-1, 1)),  // down-left + up-right
        ];

        for (dir1, dir2) in diagonal_pairs.iter() {
            let (dir1_row, dir1_col) = *dir1;
            let (dir2_row, dir2_col) = *dir2;

            // Check if we have MAS in both directions from the center
            if self.check_mas(center_row, center_col, dir1_row, dir1_col)
                && self.check_mas(center_row, center_col, dir2_row, dir2_col)
            {
                return true;
            }
        }
        false
    }
}
