use std::f64;
use std::iter::repeat;
use std::cmp;

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
pub fn segmented_sieve(end: u128, segment_size: u128) -> Vec<u128>{
    fn mark_primes(is_prime: &mut Vec<bool>, prime: &u128, segment_start: u128, segment_end: u128){
        let factor : u128 = cmp::max((segment_start  as f64 / *prime  as f64).ceil() as u128, 2);
        for i in (factor*prime..segment_end).step_by(*prime as usize){
            is_prime[(i - segment_start) as usize] = false;
        }
    }

    let end = end + end % segment_size as u128;
    let mut primes: Vec<u128> = Vec::new();

    for segment_start in(0..end).step_by(segment_size as usize) {
        let mut is_prime: Vec<bool> = repeat(true).take(segment_size as usize).collect();
        let segment_end = segment_start + segment_size;
        let mut shifted_start = segment_start;
        if segment_start == 0 {
            shifted_start = 2;
            is_prime[0] = false; 
            is_prime[1] = false;
        }

        for prime in &primes {
            mark_primes(&mut is_prime, prime, segment_start, segment_end)
        }

        for i in shifted_start..(segment_end as f64).sqrt() as u128 + 1{
            if is_prime[(i - segment_start) as usize] {
                mark_primes(&mut is_prime, &i, segment_start, segment_end);
            }
        }

        primes.extend(is_prime.iter().enumerate()
                        .filter(|(_, x)| **x)
                        .map(|(i, _)| i as u128 + segment_start));

    }
    primes
}

// Linear Sieve
pub fn linear_sieve(end: u128) -> Vec<u128>{
    let mut primes: Vec<u128> = Vec::new();
    let mut is_primes: Vec<u128> = repeat(0).take(end as usize + 1).collect();
    
    for candidate in 2..end{
        match is_primes[candidate as usize] {
            0 => {
                is_primes[candidate as usize] = candidate;
                primes.push(candidate);
             }
            _ => {},
        }
        for smaller_prime in primes.iter().filter(|x| **x <= candidate) {
            let not_a_prime = candidate * smaller_prime;
            if not_a_prime <= end {
                is_primes[(candidate * smaller_prime) as usize] = *smaller_prime;
            } 

        }
    }
    primes

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eratosthenes(){
        let end = 10;
        let primes = sieve_eratosthenes(end);
        assert_eq!(primes, vec![2, 3, 5, 7])
    }

    #[test]
    fn test_segmented_sieve(){
        let end = 10;
        let primes = segmented_sieve(end, 2);
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_linear_sieve(){
        let end = 10;
        let primes = linear_sieve(end);
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }
}