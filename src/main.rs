use prime_sieve::sieve_eratosthenes;
fn main() {
    let primes = sieve_eratosthenes(10);
    println!("{:?}" ,primes);
}
