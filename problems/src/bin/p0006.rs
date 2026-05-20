fn sum_square_difference(n: u64) -> u64 {
    let sum_squares: u64 = (1..=n).map(|n| n.pow(2)).sum();
    let square_sum = (1..=n).sum::<u64>().pow(2);
    square_sum - sum_squares
}

fn main() {
    let result = sum_square_difference(100);
    println!("{result}");
}
