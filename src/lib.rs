//! # Condition Utils
//! 
//! 

/// Define functions to compare if a value lies within a range.
pub trait Between
where
    Self: PartialEq + PartialOrd + Sized,
{
    /// Number lies between 2 values, both included. Assumes left is smaller than right.
    fn between(&self, left: Self, right: Self) -> bool {
        *self >= left && *self <= right
    }

    /// Number lies within 2 values, both excluded. Assumes left is smaller than right.
    fn within(&self, left: Self, right: Self) -> bool {
        *self > left && *self < right
    }

    /// Number lies between 2 values, left included, right excluded. Assumes left is smaller than right.
    fn leftween(&self, left: Self, right: Self) -> bool {
        *self >= left && *self < right
    }

    /// Number lies between 2 values, right included, left excluded. Assumes left is smaller than right.
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

    /// Number lies between 2 values, both included. If left is bigger than right, swap order.
    fn ord_between(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self >= left && self <= right
    }

    /// Number lies within 2 values, both excluded. If left is bigger than right, swap order.
    fn ord_within(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self > left && self < right
    }

    /// Number lies between 2 values, left included, right excluded. If left is bigger than right, swap order.
    fn ord_leftween(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(&right);
        self >= left && self < right
    }

    /// Number lies between 2 values, right included, left excluded. If left is bigger than right, swap order.
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
    Self: PartialEq + Sized,
{
    /// Array contains the value.
    fn is_in(&self, set: &[Self]) -> bool {
        if let None = set.iter().find(|&x| x == self) {
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
    }

    #[test]
    fn test_in() {
        assert!(10.is_in(&[1, 4, 10, 0]));
        assert!('G'.is_in(&['A', 'G', 'z']));
        assert!("Red".is_in(&vec!["Green", "Blue", "Grey", "Red", "Purple"]));
    }
}