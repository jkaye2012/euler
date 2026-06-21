use shared::primes::primes_up_to;

fn main() {
    let result: usize = primes_up_to(1_999_999).iter().sum();
    println!("{result}");
}
