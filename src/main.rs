use prime_sieve::sieve_eratosthenes;
use prime_sieve::segmented_sieve;


fn main() {
    let end: u128 = 100;
    let primes = sieve_eratosthenes(end);
    println!("SIEVE OF ERASTHENES \n");
    println!("PRIMES UNTIL {end} \n");
    print!("{:?} \n", primes);

    let primes = segmented_sieve(end, 20);
    print!("SEGMENTED SIEVE \n");
    print!("PRIMES UNTIL {end} \n");
    println!("{:?}", primes);

}
