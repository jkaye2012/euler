fn is_palindrome(n: u64) -> bool {
    let mut copy = n;
    let mut reversed = 0u64;
    while copy > 0 {
        let digit = copy % 10;
        copy /= 10;
        reversed *= 10;
        reversed += digit;
    }

    reversed == n
}

fn main() {
    let mut max = 0;
    for i in 100..=999 {
        for j in 100..=999 {
            let product = i * j;
            if is_palindrome(product) && product > max{
                max = product;
            }
        }
    }
    println!("{max}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(1221));
        assert!(is_palindrome(900009));
        assert!(is_palindrome(909909));
        assert!(!is_palindrome(12345));
    }
}
