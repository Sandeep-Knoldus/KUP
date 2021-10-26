/// Struct GeometricSeries stores the variables
///
/// #Field
///
/// first_number: i32 type integer object for the first number in GP series
/// ratio: i32 type integer object for the common ratio in GP series
/// current_number: i32 type integer object for the current number in GP series
pub struct GeometricSeries {
    pub first_number: i32,
    pub ratio: i32,
    pub current_number: i32,
}

/// Trait Iterator defines function 'generate' in an abstract way
pub trait Iterator {
    /// Function 'take' generates the GP series
    ///
    /// #Arguments
    ///
    /// size: i32 type integer object for the generating upto the nth GP series
    ///
    /// #Return
    ///
    /// Returns Vec<i32> having the generated GP series
    fn take(&mut self, size: i32) -> Vec<i32>;
}

impl Iterator for GeometricSeries {
    /// Function 'take' generates the GP series
    ///
    /// #Arguments
    ///
    /// size: i32 type integer object for the generating upto the nth GP series
    ///
    /// #Return
    ///
    /// Returns Vec<i32> having the generated GP series
    fn take(&mut self, size: i32) -> Vec<i32> {
        let mut gp_series: Vec<i32> = Vec::new();
        for _loop1 in 0..size {
            gp_series.push(self.first_number);
            self.first_number *= self.ratio;
        }
        gp_series
    }
}
