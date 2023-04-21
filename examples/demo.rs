use cond_utils::{Between, In};

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
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.module().partial_cmp(&other.module())
    }
}

fn main() {
    dbg!(10.between(1, 20));
    dbg!(10.between(1, 10));
    dbg!(10.within(1, 10));
    dbg!(10.between(0, 9));
    dbg!(10.ord_between(10, 0));
    dbg!("Asllop".to_owned().between("Abc".to_owned(), "Bca".to_owned()));
    dbg!('g'.between('a', 'z'));
    dbg!('G'.is_in(&['A', 'G', 'z']));
    dbg!("Red".is_in(&vec!["Green", "Blue", "Grey", "Red", "Purple"]));
    dbg!(10.in_ranges(&[0..5, 20..1999, 5..12]));
    // custom type
    dbg!(Complex::new(1.0, 1.0).between(Complex::new(0.0, 1.0), Complex::new(2.0, 2.0)));
    dbg!(Complex::new(1.0, 1.0).between(Complex::new(0.0, 1.0), Complex::new(1.0, 1.0)));
    dbg!(Complex::new(1.0, 1.0).within(Complex::new(0.0, 1.0), Complex::new(1.0, 1.0)));
}
