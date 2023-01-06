use std::f64;
use std::iter::repeat;

/// Sieve of Eratosthenes
pub fn sieve_eratosthenes(end: u128) -> Vec<u128>{
    let mut is_prime: Vec<bool> = repeat(true).take(end as usize).collect();
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

/// Segmented Sieve
pub fn segmented_sieve(end: u128, segment_size: u128){
    fn mark_primes(is_prime: &mut Vec<bool>, prime: &u128, segment_start: u128, segment_end: u128){
        let factor : u128 = (segment_start  as f64 / *prime  as f64).ceil() as u128;
        for i in (factor*prime..segment_end+1).step_by(*prime as usize){
            is_prime[(i - segment_start) as usize] = false;
        }
    }

    let end = end + end % segment_size as u128;
    let mut primes: Vec<u128> = Vec::new();

    for mut segment_start in(0..end+1).step_by(segment_size as usize) {
        let mut is_prime: Vec<bool> = repeat(true).take(segment_size as usize).collect();
        let segment_end = segment_start + segment_size;
        if segment_start == 0 {
            segment_start = 2;
            is_prime[0] = false; 
            is_prime[1] = false;
        }

        for prime in &primes {
            mark_primes(&mut is_prime, prime, segment_start, segment_end)
        }
        for i in segment_start..(segment_end as f64).sqrt() as u128 {
            if is_prime[i as usize] {
                mark_primes(&mut is_prime, &i, segment_start, segment_end);
            }
        }

    }

}