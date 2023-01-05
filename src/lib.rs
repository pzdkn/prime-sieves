use std::f64;
use std::iter::repeat;

pub fn sieve_eratosthenes(end: u128) -> Vec<u128>{
    let mut is_prime: Vec<bool> = repeat(true).take((end) as usize).collect();
    is_prime[0] = false;

    for i in 2..(f64::sqrt(end as f64) as u128) + 1 {
        if is_prime[(i - 1) as usize] {
            for j in (2*i ..end +1).step_by((i) as usize) {
                is_prime[(j - 1) as usize] = false;
            }
        } 
    }
   is_prime.iter().enumerate().filter(|(_, x)| **x).map(|(i, _) | i as u128 +1).collect()
}