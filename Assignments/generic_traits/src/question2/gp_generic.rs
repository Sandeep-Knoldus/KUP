/// Trait Iterator defines function 'generate' in an abstract way
pub trait Iterator {
    /// Function 'generate' generates the GP series
    ///
    /// #Arguments
    ///
    /// first_number: Taking i32 as input
    /// ratio: Taking i32 as input
    /// length: Taking i32 as input
    ///
    /// #Return
    ///
    /// Returns Vec<i32> having the generated GP series
    fn generate(first_number: i32, ratio: i32, length: i32) -> Vec<i32>;
}

/// Struct GeometricSeries stores the variables
///
/// #Field
///
/// first_number: i32 type integer object for the first number in GP series
/// ratio: i32 type integer object for the common ratio in GP series
/// length: i32 type integer object for the generating upto the nth GP series
pub struct GeometricSeries {
    pub first_number: i32,
    pub ratio: i32,
    pub length: i32,
}

impl Iterator for GeometricSeries {
    /// Function 'generate' generates the GP series
    ///
    /// #Arguments
    ///
    /// first_number: Taking i32 as input
    /// ratio: Taking i32 as input
    /// length: Taking i32 as input
    ///
    /// #Return
    ///
    /// Returns Vec<i32> having the generated GP series
    fn generate(first_number: i32, ratio: i32, length: i32) -> Vec<i32> {
        let mut curr: i32;
        let mut vec: Vec<i32> = Vec::new();
        for loop1 in 0..length as usize {
            curr = first_number * (power1(ratio, loop1 as i32));
            vec.push(curr);
        }
        vec
    }
}

/// Function 'power1' calculates base^power in a recursive way
/// 
/// #Arguments
/// 
/// base: Taking i32 as input from 'generate' function
/// power: Taking i32 as input from 'generate' function
/// 
/// #Return
/// 
/// Returns i32 calculating base^power 
pub fn power1(base: i32, power: i32) -> i32 {
    if power == 0 {
        return 1;
    }
    if power % 2 == 0 {
        let const1 = power1(base, power / 2);
        const1 * const1
    } else {
        base * power1(base, power - 1)
    }
}
