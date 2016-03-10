pub struct ElectroMagnetic1D {
    space_size: usize,
    pub ex: Vec<f64>,
    pub hy: Vec<f64>,
    pub cb: Vec<f64>,
    pub ex_low_m: [f64; 2],
    pub ex_high_m: [f64; 2]
}

impl ElectroMagnetic1D {

    /// Constructs a new, zero filled `ElectroMagnetic1D` with the specified size.
    /// The dielectric constant for the field is set to 0.5 as a default.
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
            hy: vec![0.0; size],
            cb: vec![0.5; size],
            ex_low_m: [0.0; 2],
            ex_high_m: [0.0; 2]
        }
    }

    pub fn tick_ex(&mut self) {
        // Calculate the Ex field.
        for k in 1..self.space_size {
            self.ex[k] = self.ex[k] + self.cb[k] * (self.hy[k-1] - self.hy[k]);
        }
        
        /* Absorbing Boundary Conditions */
        // Low Boundary Condition.
        self.ex[0] = self.ex_low_m[1];
        self.ex_low_m[1] = self.ex_low_m[0];
        self.ex_low_m[0] = self.ex[1];

        // High Boundary Condition.
        self.ex[self.space_size-1] = self.ex_high_m[1];
        self.ex_high_m[1] = self.ex_high_m[0];
        self.ex_high_m[0] = self.ex[self.space_size - 2];
    }

    pub fn tick_hy(&mut self) {



        // Calculate the Hy field.
        for k in 0..self.len() - 1 {
            self.hy[k] = self.hy[k] + 0.5f64 * (self.ex[k] - self.ex[k+1]);
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
