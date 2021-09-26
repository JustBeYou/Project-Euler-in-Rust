use crate::algo::primes::{Sieve};

pub fn solve(n: usize) -> usize {
    const LIMIT: usize = 10_000;
    let sieve: Sieve<LIMIT> = Sieve::new();

    for i in (2..LIMIT).rev() {
        if sieve.is_prime(i) && n % i == 0 {
            return i;
        }
    }

    return 0;
}