/*
number types -> between(a,b)
all types -> in(a,b,c,d,e,...)
*/

/// Define functions to compare if a number lies within a range.
pub trait Between where
    Self: PartialEq + PartialOrd + Sized + Copy {
    
    /// Number lies between 2 values, both included. Assumes left is smaller than right.
    fn between(&self, left: Self, right: Self) -> bool {
        return *self >= left && *self <= right;
    }

    /// Number lies within 2 values, both excluded. Assumes left is smaller than right.
    fn within(&self, left: Self, right: Self) -> bool {
        return *self > left && *self < right;
    }

    /// Number lies between 2 values, left included, right excluded. Assumes left is smaller than right.
    fn leftween(&self, left: Self, right: Self) -> bool {
        return *self >= left && *self < right;
    }

    /// Number lies between 2 values, right included, left excluded. Assumes left is smaller than right.
    fn rightween(&self, left: Self, right: Self) -> bool {
        return *self > left && *self <= right;
    }

    /// Reorder a range to accomply that left value is smaller than right.
    fn reorder(&self, right: Self) -> (Self, Self) {
        if *self > right {
            (right, *self)
        }
        else {
            (*self, right)
        }
    }

    /// Number lies between 2 values, both included. If left is bigger than right, swap order.
    fn ord_between(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(right);
        self.between(left, right)
    }

    /// Number lies within 2 values, both excluded. If left is bigger than right, swap order.
    fn ord_within(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(right);
        self.within(left, right)
    }

    /// Number lies between 2 values, left included, right excluded. If left is bigger than right, swap order.
    fn ord_leftween(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(right);
        self.leftween(left, right)
    }

    /// Number lies between 2 values, right included, left excluded. If left is bigger than right, swap order.
    fn ord_rightween(&self, left: Self, right: Self) -> bool {
        let (left, right) = left.reorder(right);
        self.rightween(left, right)
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
