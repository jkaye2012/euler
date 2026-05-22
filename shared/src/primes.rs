/// Sieve of Eratosthenes. Returns a Vec<bool> where index `i` is true iff `i` is prime.
pub fn sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    if limit >= 1 {
        is_prime[0] = false;
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

/// Returns the nth prime number, where n is a 1-based index (the 1st prime number is 2).
///
/// ```
/// use shared::primes::nth_prime;
/// assert_eq!(nth_prime(6), 13);
/// assert_eq!(nth_prime(7), 17);
/// assert_eq!(nth_prime(8), 19);
/// ```
pub fn nth_prime(n: usize) -> usize {
    let nf = n as f64;
    let upper_bound = (nf * (nf.ln() + nf.ln().ln())) as usize;
    let s = sieve(upper_bound);
    s.iter()
        .enumerate()
        .filter(|(_, prime)| **prime)
        .take(n)
        .last()
        .unwrap()
        .0
}

/// Returns all primes up to and including `limit`.
pub fn primes_up_to(limit: usize) -> Vec<usize> {
    sieve(limit)
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| if p { Some(i) } else { None })
        .collect()
}

/// Returns all prime factors of `target`; relatively inefficient brute-force method
pub fn prime_factors(mut target: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for num in 2..target / 2 {
        if is_prime(num) && target % num == 0 {
            target /= num;
            result.push(num);
            if target == 1 {
                break;
            }
        }
    }

    result
}

/// Trial-division primality test. Fine for small n; use a sieve for bulk work.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i: u64 = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
