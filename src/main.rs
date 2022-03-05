use cond_utils::{Between, In};

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
}