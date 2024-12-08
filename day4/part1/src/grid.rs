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

    pub fn check_xmas_at_position(
        &self,
        start_row: i32,
        start_col: i32,
        dir_row: i32,
        dir_col: i32,
    ) -> bool {
        let target = ['X', 'M', 'A', 'S'];

        for i in 0..4 {
            let row = start_row + i * dir_row;
            let col = start_col + i * dir_col;

            match self.get_char(row, col) {
                Some(c) if c == target[i as usize] => continue,
                _ => return false,
            }
        }
        true
    }
}
