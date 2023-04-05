struct Image {
    pixels: Vec<u32>,
    width: usize,
}

impl Image {
    #[allow(dead_code)]
    fn new(pixel_data: Vec<u32>, width: usize) -> Self {
        Image {
            pixels: pixel_data,
            width,
        }
    }

    #[allow(dead_code)]
    fn rotate_clockwise(&mut self) {
        self.transpose();
        self.inverse_columns();
    }

    fn transpose(&mut self) {
        for col in 0..self.width {
            for row in (col + 1)..self.width {
                self.pixels
                    .swap(col + row * self.width, row + col * self.width);
            }
        }
    }
    fn inverse_columns(&mut self) {
        for col in 0..(self.width / 2) {
            for row in 0..self.width {
                self.pixels.swap(
                    col + row * self.width,
                    (self.width - 1 - col) + row * self.width,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_transpose_image() {
        let mut image = Image::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        image.transpose();
        assert_eq!(image.pixels, vec![1, 4, 7, 2, 5, 8, 3, 6, 9]);
    }

    #[test]
    fn should_inverse_column_in_image() {
        let mut image = Image::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        image.inverse_columns();
        assert_eq!(image.pixels, vec![3, 2, 1, 6, 5, 4, 9, 8, 7]);
    }

    #[test]
    fn should_rotate_image_clockwise() {
        let mut image = Image::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        image.rotate_clockwise();
        assert_eq!(image.pixels, vec![7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
