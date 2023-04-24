use cond_utils::{Between, In};

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
    assert!("Asllop".between("Abc", "Bca"));
    assert!(Box::new(10).between(Box::new(5), Box::new(15)) == true);
    assert!("Asllop"
        .to_owned()
        .between("Abc".to_owned(), "Bca".to_owned()));
}

struct Complex {
    real: f64,
    imag: f64,
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

#[test]
fn test_custom_type() {
    assert!(Complex::new(1.0, 1.0).between(Complex::new(0.0, 1.0), Complex::new(2.0, 2.0)) == true);
    assert!(
        Complex::new(1.0, 1.0).between(Complex::new(0.0, 1.0), Complex::new(1.0, 1.0)) == false
    );
    assert!(Complex::new(1.0, 1.0).within(Complex::new(0.0, 1.0), Complex::new(1.0, 1.0)) == true);
}

#[test]
fn test_in() {
    assert!(10.is_in(&[1, 4, 10, 0]));
    assert!('G'.is_in(&['A', 'G', 'z']));
    assert!("Red".is_in(&vec!["Green", "Blue", "Grey", "Red", "Purple"]));
    assert!(10.in_ranges(&[0..5, 20..1999, 5..12]));
}
