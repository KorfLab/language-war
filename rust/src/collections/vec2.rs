use std::collections::HashMap;
use std::hash::Hash;

/// A simple, heap-allocated, row-major matrix using a flat Vec.
#[derive(Debug, Clone)]
pub struct Vec2<T> {
    data: Vec<T>,
    cols: usize,
}

/// Read-only keyed view over a `Vec2<T>` matrix.
///
/// Borrows the matrix data and index maps, enabling
/// `get(row_key, col_key)` lookups without owning any data.
pub struct IndexedVec2<'a, R, C, T>
where
    R: Eq + Hash,
    C: Eq + Hash,
{
    data: &'a Vec2<T>,
    row_indices: &'a HashMap<R, usize>,
    col_indices: &'a HashMap<C, usize>,
}

impl<'a, R, C, T> IndexedVec2<'a, R, C, T>
where
    R: Eq + Hash,
    C: Eq + Hash,
{
    pub fn new(
        data: &'a Vec2<T>,
        row_indices: &'a HashMap<R, usize>,
        col_indices: &'a HashMap<C, usize>,
    ) -> Self {
        Self {
            data,
            row_indices,
            col_indices,
        }
    }

    pub fn get(&self, row_key: &R, col_key: &C) -> Option<&T> {
        let row = self.row_indices.get(row_key)?;
        let col = self.col_indices.get(col_key)?;
        Some(self.data.get(*row, *col))
    }

    pub fn row_index(&self, key: &R) -> Option<usize> {
        self.row_indices.get(key).copied()
    }

    pub fn col_index(&self, key: &C) -> Option<usize> {
        self.col_indices.get(key).copied()
    }
}

impl<T> Vec2<T> {
    /// Creates a new matrix with specified dimensions filled with a default value.
    pub fn with_default(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; rows * cols],
            cols,
        }
    }

    /// Creates a new matrix with specified dimensions filled with default value
    /// of that type.
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            data: vec![T::default(); rows * cols],
            cols,
        }
    }

    /// Creates a matrix from a flat vector and dimensions.
    pub fn from_flat(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols, "Dimension mismatch");
        Self { data, cols }
    }

    /// Safe access to an element.
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    /// Mutable access to an element.
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Helper for debugging or printing.
    pub fn row(&self, row: usize) -> &[T] {
        let start = row * self.cols;
        &self.data[start..start + self.cols]
    }
}
