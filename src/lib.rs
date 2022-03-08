//! # Condition Utils
//! 
//! It is a very simple crate that provides two traits with comparation utils: [Between] and [In]. It also implements the forementioned traits for the following types:
//! 
//! - [i8]
//! - [u8]
//! - [i16]
//! - [u16]
//! - [i32]
//! - [u32]
//! - [i64]
//! - [u64]
//! - [i128]
//! - [u128]
//! - [isize]
//! - [usize]
//! - [f32]
//! - [f64]
//! - [char]
//! - [&str]
//! - [String]
//! 
//! The objective of `cond_utils` is to simplify and make more legible some common tasks, like comparing if a value lies between 2 values or checking if a value is in a set. This allows to write code like:
//! 
//! ```
//! use cond_utils::Between;
//! 
//! let number = 6;
//! if number.between(0,10) {
//!     println!("Number is between 0 and 10");
//! }
//! ```
//! instead of:
//! 
//! ```
//! let number = 6;
//! if number >= 0 && number <= 10 {
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
//! if number.is_in(&[2,6,12]) {
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
//! if number.in_ranges(&[0..5,10..100]) {
//!     println!("Number is between 0 and 5 or between 10 and 100");
//! }
//! ```
//! 
//! instead of:
//! 
//! ```
//! let number = 6;
//! if (number >= 0 && number <= 5) || (number >= 10 && number <= 100) {
//!     println!("Number is between 0 and 5 or between 10 and 100");
//! }
//! ```

use std::ops::Range;

/// Define functions to compare if a value lies within a range.
pub trait Between
where
    Self: PartialEq + PartialOrd + Sized,
{
    /// Value lies between 2 values, both included. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.between(0,10));
    /// assert!(10.between(0,9) == false);
    /// ```
    fn between(&self, left: Self, right: Self) -> bool {
        *self >= left && *self <= right
    }

    /// Value lies within 2 values, both excluded. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(5.within(0,10));
    /// assert!(5.within(0,5) == false);
    /// ```
    fn within(&self, left: Self, right: Self) -> bool {
        *self > left && *self < right
    }

    /// Value lies between 2 values, left included, right excluded. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(0.leftween(0,10));
    /// assert!(10.leftween(0,10) == false);
    /// ```
    fn leftween(&self, left: Self, right: Self) -> bool {
        *self >= left && *self < right
    }

    /// Value lies between 2 values, right included, left excluded. Assumes left is smaller than right.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.rightween(0,10));
    /// assert!(0.rightween(0,10) == false);
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

    /// Value lies between 2 values, both included. If left is bigger than right, swap order.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.ord_between(10,0));
    /// assert!(10.ord_between(9,0) == false);
    /// ```
    fn ord_between(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self >= left && self <= right
    }

    /// Value lies within 2 values, both excluded. If left is bigger than right, swap order.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(5.ord_within(10,0));
    /// assert!(5.ord_within(5,0) == false);
    /// ```
    fn ord_within(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self > left && self < right
    }

    /// Value lies between 2 values, left included, right excluded. If left is bigger than right, swap order.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(0.ord_leftween(10,0));
    /// assert!(10.ord_leftween(10,0) == false);
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
    /// assert!(10.ord_rightween(10,0));
    /// assert!(0.ord_rightween(10,0) == false);
    /// ```
    fn ord_rightween(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self > left && self <= right
    }
}

impl Between for i8 {}
impl Between for u8 {}
impl Between for i16 {}
impl Between for u16 {}
impl Between for i32 {}
impl Between for u32 {}
impl Between for i64 {}
impl Between for u64 {}
impl Between for i128 {}
impl Between for u128 {}
impl Between for isize {}
impl Between for usize {}
impl Between for f32 {}
impl Between for f64 {}
impl Between for char {}
impl Between for &str {}
impl Between for String {}

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
    /// assert!(10.is_in(&[0,10,100]));
    /// assert!(10.is_in(&[0,100]) == false);
    /// ```
    fn is_in(&self, set: &[Self]) -> bool {
        if let None = set.iter().find(|&x| x == self) {
            false
        }
        else {
            true
        }
    }

    /// Value lies between one of the ranges.
    /// 
    /// Example:
    /// ```
    /// # use cond_utils::*;
    /// assert!(10.in_ranges(&[0..5,10..100]));
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

impl In for i8 {}
impl In for u8 {}
impl In for i16 {}
impl In for u16 {}
impl In for i32 {}
impl In for u32 {}
impl In for i64 {}
impl In for u64 {}
impl In for i128 {}
impl In for u128 {}
impl In for isize {}
impl In for usize {}
impl In for f32 {}
impl In for f64 {}
impl In for char {}
impl In for &str {}
impl In for String {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_between() {
        assert!(10.between(1, 20) == true);
        assert!(10.between(1, 10) == true);
        assert!(10.within(1, 10) == false);
        assert!(10.rightween(1, 10) == true);
        assert!(10.leftween(10, 20) == true);
        assert!(10.ord_between(20, 1) == true);
        assert!(10.ord_between(10, 1) == true);
        assert!(10.ord_within(10, 1) == false);
        assert!(10.ord_rightween(10, 1) == true);
        assert!(10.ord_leftween(20, 10) == true);
        assert!("Asllop".to_owned().between("Abc".to_owned(), "Bca".to_owned()));
    }

    #[test]
    fn test_in() {
        assert!(10.is_in(&[1, 4, 10, 0]));
        assert!('G'.is_in(&['A', 'G', 'z']));
        assert!("Red".is_in(&vec!["Green", "Blue", "Grey", "Red", "Purple"]));
        assert!(10.in_ranges(&[0..5, 20..1999, 5..12]));
    }
}