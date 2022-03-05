# Condition Utils

It is a very simple crate that provides two traits with comparation utils: `Between` and `In`. It also implements the forementioned traits for the following types:

- `i8`
- `u8`
- `i16`
- `u16`
- `i32`
- `u32`
- `i64`
- `u64`
- `i128`
- `u128`
- `isize`
- `usize`
- `f32`
- `f64`
- `char`
- `&str`
- `String`

The objective of `cond_utils` is to simplify and make more legible some common tasks, specifically two: comparing if a value lies between 2 values and checking if a value is in a set. This allows to write code like:

```Rust
use cond_utils::Between;

let number = 6;
if number.between(0,10) {
    println!("Number is between 0 and 10");
}
```
instead of:

```Rust
let number = 6;
if number >= 0 && number <= 10 {
    println!("Number is between 0 and 10");
}
```

Or this:

```Rust
use cond_utils::In;

let number = 6;
if number.is_in(&`2,6,12`) {
    println!("Number is in set");
}
```

instead of:

```Rust
let number = 6;
if number == 2 || number == 6 || number == 12 {
    println!("Number is in set");
}
```