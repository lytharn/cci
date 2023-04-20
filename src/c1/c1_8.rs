use std::collections::HashSet;

struct Matrix {
    values: Vec<u32>,
    cols: usize,
    rows: usize,
}

impl Matrix {
    #[allow(dead_code)]
    fn new(values: Vec<u32>, cols: usize, rows: usize) -> Self {
        Self { values, cols, rows }
    }

    #[allow(dead_code)]
    fn zero_whole_row_col(&mut self) {
        let mut cols_with_zero = HashSet::new();
        let mut rows_with_zero = HashSet::new();
        for col in 0..self.cols {
            for row in 0..self.rows {
                let value = self.values[col + row * self.cols];
                if value == 0 {
                    cols_with_zero.insert(col);
                    rows_with_zero.insert(row);
                }
            }
        }
        for col in cols_with_zero {
            for row in 0..self.rows {
                self.values[col + row * self.cols] = 0;
            }
        }
        for row in rows_with_zero {
            for col in 0..self.cols {
                self.values[col + row * self.cols] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_zero_whole_row_and_column_if_zero_element() {
        let mut matrix = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 4, 3);
        matrix.zero_whole_row_col();
        assert_eq!(matrix.values, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

        let mut matrix = Matrix::new(vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 4, 3);
        matrix.zero_whole_row_col();
        assert_eq!(matrix.values, vec![0, 0, 0, 0, 0, 6, 7, 8, 0, 10, 11, 12]);

        let mut matrix = Matrix::new(vec![1, 2, 0, 4, 5, 6, 7, 0, 9, 10, 11, 12], 4, 3);
        matrix.zero_whole_row_col();
        assert_eq!(matrix.values, vec![0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 0, 0]);
    }
}
