use cond_utils::{Between, In};

fn main() {
    dbg!(10.between(1, 20));
    dbg!(10.between(1, 10));
    dbg!(10.between(0, 9));
    dbg!(10.ord_between(10, 0));
    dbg!('g'.between('a', 'z'));
    dbg!('G'.is_in(&['A', 'G', 'z']));
}