//! Even_odd_checker
//!
//! This is a library for finding even or odd number given by cmd.

/// Find number even or odd
///
/// #Example
/// let's suppose you enter 5 in cmd arguments -
///
/// ```
///
/// let arg =5;
/// let answer = Even_odd_checker::even_odd_checker::check_even_odd(arg);
///
/// assert_eq!("odd", answer);
/// ```
// this is comment
pub use self::even_odd_checker::check_even_odd;

pub mod even_odd_checker {
    use colored::*;
    pub enum Numtype {
        Even,
        Odd,
    }

    pub fn check_even_odd(num: u32) -> &'static str {
        if num <= 0 {
            panic!("{}", "Invalid number , input greater than 0!".red());
        }
        match num % 2 == 0 {
            true => "even",
            false => "odd",
        }
    }
}
