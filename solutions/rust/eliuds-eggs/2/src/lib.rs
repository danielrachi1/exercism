//! Count the number of 1 bits in the binary represtantion of a number.
//!
//! ### Problem breakdown
//!
//! Since we can't use the standard library, the problem can be broken
//! into two steps.
//!
//! 1. Get the binary representation of a number.
//! 2. Count the 1s in that binary.
//!
//! ### Algorithm
//!
//! The int to binary conversion is easily achieved with integer division and
//! modulo operations.
//!
//! 1. Get the binary representation of a number.
//!    1.1. Find and save the modulo 2 of the number.
//!    1.2. Divide by two.
//!    1.3. Take the result and go back to 1.1. Repeat until the number is 0.
//!    1.4. Invert the list of remainder. (Actually not needed here.)
//! 2. Count the 1s in the list.

pub fn egg_count(display_value: u32) -> usize {
    let mut n = display_value;
    let mut remainders = Vec::new();

    while n != 0 {
        remainders.push(n % 2);
        n /= 2;
    }

    remainders.into_iter().filter(|digit| *digit == 1).count()
}
