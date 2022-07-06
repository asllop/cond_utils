//! # Condition Utils
//! 
//! It is a very simple crate that provides two traits with comparation utils: [Between] and [In].
//! 
//! The objective of `cond_utils` is to simplify and make more legible some common tasks, like comparing if a value lies between 2 values or checking if a value is in a set. This allows to write code like:
//! 
//! ```
//! use cond_utils::Between;
//! 
//! let number = 6;
//! if number.between(0, 10) {
//!     println!("Number is between 0 and 10");
//! }
//! ```
//! instead of:
//! 
//! ```
//! let number = 6;
//! if number > 0 && number < 10 {
//!     println!("Number is between 0 and 10");
//! }
//! ```
//! 
//! This:
//! 
//! ```
//! use cond_utils::In;
//! 
//! let number = 6;
//! if number.is_in(&[2, 6, 12]) {
//!     println!("Number is in set");
//! }
//! ```
//! 
//! instead of:
//! 
//! ```
//! let number = 6;
//! if number == 2 || number == 6 || number == 12 {
//!     println!("Number is in set");
//! }
//! ```
//! 
//! Or this:
//! 
//! ```
//! use cond_utils::In;
//! 
//! let number = 6;
//! if number.in_ranges(&[0..5, 10..100]) {
//!     println!("Number is between 0 and 5 or between 10 and 100, limits included");
//! }
//! ```
//! 
//! instead of:
//! 
//! ```
//! let number = 6;
//! if (number >= 0 && number <= 5) || (number >= 10 && number <= 100) {
//!     println!("Number is between 0 and 5 or between 10 and 100, limits included");
//! }
//! ```

use std::ops::Range;

/// Define functions to compare if a value belongs to range.
pub trait Between
where
    Self: PartialEq + PartialOrd + Sized,
{
    /// Value lies between 2 values, both excluded. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.between(0, 11));
    /// assert!(10.between(0, 10) == false);
    /// ```
    fn between(&self, left: Self, right: Self) -> bool {
        *self > left && *self < right
    }

    /// Value lies within 2 values, both included. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(5.within(0, 10));
    /// assert!(6.within(0, 5) == false);
    /// ```
    fn within(&self, left: Self, right: Self) -> bool {
        *self >= left && *self <= right
    }

    /// Value lies between 2 values, left included, right excluded. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(0.leftween(0, 10));
    /// assert!(10.leftween(0, 10) == false);
    /// ```
    fn leftween(&self, left: Self, right: Self) -> bool {
        *self >= left && *self < right
    }

    /// Value lies between 2 values, right included, left excluded. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.rightween(0, 10));
    /// assert!(0.rightween(0, 10) == false);
    /// ```
    fn rightween(&self, left: Self, right: Self) -> bool {
        *self > left && *self <= right
    }

    /// Reorder a range to accomply that left value is smaller than right.
    fn reorder<'a>(&'a self, right: &'a Self) -> (&'a Self, &'a Self) {
        if self > right {
            (right, self)
        }
        else {
            (self, right)
        }
    }

    /// Value lies between 2 values, both excluded. If left is bigger than right, swap order.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.ord_between(11, 0));
    /// assert!(10.ord_between(10, 0) == false);
    /// ```
    fn ord_between(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self > left && self < right
    }

    /// Value lies within 2 values, both included. If left is bigger than right, swap order.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(5.ord_within(10, 0));
    /// assert!(6.ord_within(5, 0) == false);
    /// ```
    fn ord_within(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self >= left && self <= right
    }

    /// Value lies between 2 values, left included, right excluded. If left is bigger than right, swap order.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(0.ord_leftween(10, 0));
    /// assert!(10.ord_leftween(10, 0) == false);
    /// ```
    fn ord_leftween(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self >= left && self < right
    }

    /// Value lies between 2 values, right included, left excluded. If left is bigger than right, swap order.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.ord_rightween(10, 0));
    /// assert!(0.ord_rightween(10, 0) == false);
    /// ```
    fn ord_rightween(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self > left && self <= right
    }
}

/// Define functions to compare if a value belongs to a set.
pub trait In
where
    Self: PartialEq + PartialOrd + Sized,
{
    /// Array contains the value.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.is_in(&[0, 10, 100]));
    /// assert!(10.is_in(&[0, 100]) == false);
    /// ```
    fn is_in(&self, set: &[Self]) -> bool {
        if let None = set.iter().find(|&x| x == self) {
            false
        }
        else {
            true
        }
    }

    /// Value lies within one of the ranges.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.in_ranges(&[0..5, 10..100]));
    /// assert!(10.in_ranges(&[100..1000]) == false);
    /// ```
    fn in_ranges(&self, ranges: &[Range<Self>]) -> bool {
        if let None = ranges.iter().find(|&x| x.contains(self)) {
            false
        }
        else {
            true
        }
    }
}

impl<T> Between for T where T: PartialEq + PartialOrd + Sized {}
impl<T> In for T where T: PartialEq + PartialOrd + Sized {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_between() {
        assert!(10.between(1, 20) == true);
        assert!(10.between(1, 10) == false);
        assert!(10.8_f32.between(5.5_f32, 10.9_f32));
        assert!(10.within(1, 10) == true);
        assert!(true.within(true, true));
        assert!(10.rightween(1, 10) == true);
        assert!(10.leftween(10, 20) == true);
        assert!(10.ord_between(20, 1) == true);
        assert!(10.ord_between(10, 1) == false);
        assert!(10.ord_within(10, 1) == true);
        assert!(10.ord_rightween(10, 1) == true);
        assert!(10.ord_leftween(20, 10) == true);
        assert!("Asllop".to_owned().between("Abc".to_owned(), "Bca".to_owned()));
        assert!("Asllop".between("Abc", "Bca"));
        assert!(Box::new(10).between(Box::new(5), Box::new(15)) == true);
    }

    struct Complex {
        real: f64,
        imag: f64
    }

    impl Complex {
        pub fn new(real: f64, imag: f64) -> Self {
            Self { real, imag }
        }

        pub fn module(&self) -> f64 {
            (self.real.powi(2) + self.imag.powi(2)).sqrt()
        }
    }

    impl PartialEq for Complex {
        fn eq(&self, other: &Self) -> bool {
            self.module() == other.module()
        }
    }

    impl PartialOrd for Complex {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.module().partial_cmp(&other.module())
        }
    }

    #[test]
    fn test_custom_type() {
        assert!(Complex::new(1.0, 1.0).between(Complex::new(0.0, 1.0), Complex::new(2.0, 2.0)) == true);
        assert!(Complex::new(1.0, 1.0).between(Complex::new(0.0, 1.0), Complex::new(1.0, 1.0)) == false);
        assert!(Complex::new(1.0, 1.0).within(Complex::new(0.0, 1.0), Complex::new(1.0, 1.0)) == true);
    }

    #[test]
    fn test_in() {
        assert!(10.is_in(&[1, 4, 10, 0]));
        assert!('G'.is_in(&['A', 'G', 'z']));
        assert!("Red".is_in(&vec!["Green", "Blue", "Grey", "Red", "Purple"]));
        assert!(10.in_ranges(&[0..5, 20..1999, 5..12]));
    }
}