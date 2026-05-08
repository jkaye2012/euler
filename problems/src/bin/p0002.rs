use shared::math::Fibonacci;

fn main() {
    let fib = Fibonacci::new().into_iter();
    let sum: u64 = fib
        .take_while(|x| *x <= 4000000)
        .filter(|x| *x % 2 == 0)
        .sum();
    println!("{sum}");
}
