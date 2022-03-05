use cond_utils::Between;

fn main() {
    println!("{}", 10.between(1, 20));
    println!("{}", 10.between(1, 10));
    println!("{}", 10.between(0, 9));
    println!("{}", 10.ord_between(10, 0));
}