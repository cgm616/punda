use core::ops::{Deref, DerefMut};

// (row, column)
const DISPLAY_MAP: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

/// A representation of an image to be displayed on the LED matrix.
///
/// The array is arranged so that it is an array of rows. When represented in
/// source code as follows, each line of the array corresponds to a row of LEDs.
///
/// ```
/// let image = [
///     [false, true, false, true, false], // first row
///     [false, false, false, false, false], // second row
///     [true, false, false, false, true], // third row
///     [false, true, true, true, false], // fourth row
///     [false, false, false, false, false], // fifth row
/// ]
/// ```
#[derive(Copy, Clone)]
pub struct DisplayImage(pub [[bool; 5]; 5]);

crate struct MatrixImage([[bool; 9]; 3]);

impl Deref for DisplayImage {
    type Target = [[bool; 5]; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DisplayImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for MatrixImage {
    type Target = [[bool; 9]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MatrixImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[[bool; 9]; 3]> for MatrixImage {
    fn from(other: [[bool; 9]; 3]) -> Self {
        MatrixImage(other)
    }
}

impl From<DisplayImage> for MatrixImage {
    fn from(other: DisplayImage) -> Self {
        let mut matrix: MatrixImage = MatrixImage([[false; 9]; 3]);

        for (output_row, location_row) in other.iter().zip(DISPLAY_MAP.iter()) {
            for (value, index) in output_row.iter().zip(location_row.iter()) {
                matrix[index.0][index.1] = *value;
            }
        }

        matrix
    }
}

impl From<[[bool; 5]; 5]> for DisplayImage {
    fn from(other: [[bool; 5]; 5]) -> Self {
        DisplayImage(other)
    }
}
