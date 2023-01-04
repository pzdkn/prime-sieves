use std::f64;
use std::iter::repeat;

pub fn sieve_eratosthenes(end: u128) -> Vec<u128>{
    let mut primes: Vec<u128> = vec![];
    let mut is_prime: Vec<bool> = repeat(true).take((end - 1) as usize).collect();
    for i in 2..f64::sqrt(end as f64) as u128 {
        if is_prime[i as usize] {
            primes.push(i);
            for j in (i..end).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        } 
    }
    primes
}