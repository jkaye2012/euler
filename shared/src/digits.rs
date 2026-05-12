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

/// Returns the number of digits in `n`
///
/// # Examples
///
/// ```
/// use shared::digits::num_digits;
/// assert_eq!(num_digits(9), 1);
/// assert_eq!(num_digits(123), 3);
/// assert_eq!(num_digits(1000), 4);
/// ```
pub fn num_digits(mut n: u64) -> u64 {
    let mut result = 0;
    while n > 0 {
        n /= 10;
        result += 1;
    }

    result
}

/// Returns the largest number with digit length `digits`
///
/// # Examples
///
/// ```
/// use shared::digits::largest;
/// assert_eq!(largest(1), 9);
/// assert_eq!(largest(2), 99);
/// assert_eq!(largest(3), 999);
/// ```
pub fn largest(digits: u64) -> u64 {
    let mut result = 0;
    for _ in 0..digits {
        result *= 10;
        result += 9;
    }

    result
}

/// Returns the smallest number with digit length `digits`
///
/// # Examples
///
/// ```
/// use shared::digits::smallest;
/// assert_eq!(smallest(1), 1);
/// assert_eq!(smallest(2), 10);
/// assert_eq!(smallest(3), 100);
/// ```
pub fn smallest(digits: u64) -> u64 {
    let mut result = 1;
    for _ in 1..digits {
        result *= 10;
    }
    result
}
