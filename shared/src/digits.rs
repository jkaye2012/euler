/// Digits of `n` in base 10, least significant first.
pub fn digits(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut out = Vec::new();
    while n > 0 {
        out.push((n % 10) as u8);
        n /= 10;
    }
    out
}

pub fn digit_sum(n: u64) -> u64 {
    digits(n).into_iter().map(|d| d as u64).sum()
}
