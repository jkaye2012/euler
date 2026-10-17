use shared::bigint::BigUint;

fn factorial_digit_sum(n: u32) -> u32 {
    let mut result: BigUint = 1u32.into();
    for i in 1..=n {
        result *= i;
    }
    result.iter().map(|n| *n as u32).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::factorial_digit_sum;

    #[test]
    fn test_factorial_digit_sum() {
        assert_eq!(factorial_digit_sum(10), 27);
    }
}

fn main() {
    let result = factorial_digit_sum(100);
    println!("{result}");
}
