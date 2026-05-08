// Problem 1: Sum of all multiples of 3 or 5 below 1000.
fn main() {
    let sum: u64 = (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
    println!("{sum}");
}
