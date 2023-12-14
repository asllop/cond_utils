#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, boxed::Box, vec};
use cond_utils::*;

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

#[test]
fn test_in() {
    assert!(10.is_in(&[1, 4, 10, 0]));
    assert!('G'.is_in(&['A', 'G', 'z']));
    assert!("Red".is_in(&vec!["Green", "Blue", "Grey", "Red", "Purple"]));
    assert!(10.in_ranges(&[0..5, 20..1999, 5..12]));
}
