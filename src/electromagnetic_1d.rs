pub struct ElectroMagnetic1D {
    space_size: usize,
    pub ex: Vec<f64>,
    pub hy: Vec<f64>
}

impl ElectroMagnetic1D {

    /// Constructs a new, zero filled `ElectroMagnetic1D` with the specified size.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut field = ElectroMagnetic1D::new(10);
    ///
    /// // The field has 10 elements and they are all 0.
    /// assert_eq!(field.len(), 10);
    /// ```
    pub fn new(size: usize) -> ElectroMagnetic1D {
        ElectroMagnetic1D {
            space_size: size,
            ex: vec![0.0; size],
            hy: vec![0.0; size]
        }
    }

    /// Returns the number of elements in the ElectroMagnetic1D.
    ///
    /// # Examples
    ///
    /// ```
    /// let field = ElectroMagnetic1D::new(3);
    /// assert_eq!(field.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.space_size
    }
}
