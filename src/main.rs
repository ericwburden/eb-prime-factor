//! Calculate Prime Factors
//!
//! Given a positive number (up to 18,446,744,073,709,551,615), calculate the set of 
//! prime numbers that yield the given number when multiplied together. For example,
//! 2 x 2 x 3 x 5 = 60.

use std::io::{self, Write, Error, ErrorKind};

extern crate clap;
use clap::{App, Arg};

/// Determine whether a number is prime
///
/// Given a number `n`, indicate whether that number is prime.
///
/// # Examples
///
/// ```
/// # use eb_prime_factor::is_prime;
/// assert_eq!(is_prime(59), true);
/// assert_eq!(is_prime(60), false);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    let upper_limit = (n as f64).sqrt() as u64;

    for i in 2..=upper_limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Find next prime in sequence
///
/// Given a number `n`, return the next prime number that follows `n`
///
/// # Examples
///
/// ```
/// # use eb_prime_factor::next_prime;
/// assert_eq!(next_prime(60) , 61);
/// assert_eq!(next_prime(155), 157);
/// ```
pub fn next_prime(n: u64) -> u64 {
    for i in (n+1)..=u64::MAX {
        if is_prime(i) { return i; }
    }
    2
}

/// Calculate prime factors
///
/// Given a number `n`, calculate and return the prime factors of that number
///
/// # Examples
///
/// ```
/// # use eb_prime_factor::prime_factors;
/// assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
/// assert_eq!(prime_factors(49), vec![7, 7]);
/// ```
pub fn prime_factors(n: u64) -> Vec<u64> {
    if is_prime(n) { return vec![n]; } // return prime numbers entered
    let mut out = Vec::new();
    let mut divisor = 2;
    let mut remainder = n;

    while remainder > 1 {
        if remainder % divisor == 0 {
            out.push(divisor);
            remainder /= divisor;
        } else {
            // Since a factor may be repeated, only move on once a given `divisor`
            // has been ruled out completely
            divisor = next_prime(divisor);
        }
    }

    out
}

/// Calculate and display prime factors
///
/// The main function collects the user input from invocation, using [clap]. Input is
/// converted to a u64 and passed to [prime_factors]. Output is parsed back to a 
/// string containing a space-separated list of numbers, then written to
/// [std::io::stdout].
fn main() -> Result<(), Error> {
    // Use *clap* to get input from the user and provide CLI documentation
    let matches = App::new("Get Prime Factors")
        .version("1.0")
        .author("Eric Burden <eric@ericburden.dev>")
        .arg(
            Arg::with_name("INPUT")
                .help("Calculate the prime factors of a given number")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Unwrap is safe on required arguments
    let input = matches.value_of("INPUT").unwrap();
    let number = input.parse::<u64>()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    let result = prime_factors(number);
    let result_len = result.len();

    // Need to convert results to a string for putting to stdout
    let mut out_string = String::new();
    for (idx, num) in result.into_iter().enumerate() {
        let num_str = num.to_string();
        out_string.push_str(&num_str);
        if idx < result_len - 1 {
            out_string.push(' ');
        }
    }

    // Write to stdout
    writeln!(io::stdout(), "{}", out_string)?;

    Ok(()) // Needed so I can return a Result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(59), true);
        assert_eq!(is_prime(60), false);
        assert_eq!(is_prime(181), true);
        assert_eq!(is_prime(200), false);
        assert_eq!(is_prime(271), true);
        assert_eq!(is_prime(500), false);
    }

    #[test]
    pub fn test_next_prime() {
        assert_eq!(next_prime(3)  , 5);
        assert_eq!(next_prime(60) , 61);
        assert_eq!(next_prime(155), 157);
        assert_eq!(next_prime(190), 191);
        assert_eq!(next_prime(222), 223);
        assert_eq!(next_prime(250), 251);
    }

    #[test]
    pub fn test_find_prime_factors() {
        assert_eq!(prime_factors(666666), vec![2, 3, 3, 7, 11, 13, 37]);
        assert_eq!(prime_factors(222225), vec![3, 5, 5, 2963]);
        assert_eq!(prime_factors(818181), vec![3, 3, 3, 3, 3, 7, 13, 37]);
    }
}
