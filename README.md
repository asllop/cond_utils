# Condition Utils

It is a very simple crate that provides two traits with comparation utils: `Between` and `In`.

The objective of `cond_utils` is to simplify and make more legible some common tasks, like comparing if a value lies between 2 values or checking if a value is in a set. This allows to write code like:

```Rust
use cond_utils::Between;

let number = 6;
if number.between(0, 10) {
    println!("Number is between 0 and 10");
}
```
instead of:

```Rust
let number = 6;
if number > 0 && number < 10 {
    println!("Number is between 0 and 10");
}
```

This:

```Rust
use cond_utils::In;

let number = 6;
if number.is_in(&[2, 6, 12]) {
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

Or this:

```Rust
use cond_utils::In;

let number = 6;
if number.in_ranges(&[0..5, 10..100]) {
    println!("Number is between 0 and 5 or between 10 and 100");
}
```

instead of:

```Rust
let number = 6;
if (number >= 0 && number <= 5) || (number >= 10 && number <= 100) {
    println!("Number is between 0 and 5 or between 10 and 100");
}
```

It works with any type that implements `PartialEq`, `PartialOrd`, and `Sized` traits.
