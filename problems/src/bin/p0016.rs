use shared::bigint::BigUint;

fn main() {
    let mut result = BigUint::from(1);
    let two = BigUint::from(2);
    for _ in 0..1000 {
        result *= &two;
    }
    let sum: u32 = result.iter().map(|d| *d as u32).sum();
    println!("{sum}");
}
