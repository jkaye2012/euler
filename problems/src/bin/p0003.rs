pub fn main() {
    let largest_factor = *shared::primes::prime_factors(600851475143).last().unwrap();
    println!("{largest_factor}");
}
