pub struct StaticGrid<T, const N: usize, const W: usize> {
    data: [T; N],
    width: usize,
    height: usize,
}

impl<T, const N: usize, const W: usize> StaticGrid<T, N, W> {
    /// Creates a new `StaticGrid` from the provided data.
    ///
    /// # Panics
    ///
    /// If N is not an even multiple of W.
    pub fn new(data: [T; N]) -> Self {
        assert!(N % W == 0, "N must be an even multiple of W");
        Self {
            data,
            width: W,
            height: (N / W),
        }
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the element at the given `col` and `row`.
    ///
    /// # Panics
    ///
    /// If `col + row * width` is out of bounds for the underlying data.
    pub fn at(&self, col: usize, row: usize) -> T
    where
        T: Copy,
    {
        self.data[col + row * self.width]
    }

    /// Visits every cell in the grid, combining the results into a single value.
    ///
    /// `traversal` is called with the `(col, row)` of each cell and produces a
    /// value for that cell. `reducer` folds each produced value into the running
    /// result, which starts from `T::default()`. The final accumulated value is
    /// returned.
    pub fn traverse<F, R>(&self, traversal: F, reducer: R) -> T
    where
        T: Default,
        F: Fn(usize, usize) -> T,
        R: Fn(T, T) -> T,
    {
        let mut result = T::default();
        for c in 0..self.width {
            for r in 0..self.height {
                result = reducer(result, traversal(c, r));
            }
        }
        result
    }
}
