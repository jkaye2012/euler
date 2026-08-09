pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

pub struct Fibonacci {
    prev: u64,
    curr: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { prev: 0, curr: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev + self.curr;
        self.prev = self.curr;
        self.curr = next;
        Some(next)
    }
}

pub struct TriangleNumbers {
    curr: u64,
    curr_term: u64,
}

impl TriangleNumbers {
    pub fn new() -> Self {
        Self {
            curr: 1,
            curr_term: 2,
        }
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = self.curr + self.curr_term;
        self.curr_term += 1;
        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_values() {
        let mut fib = Fibonacci::new();
        for expected in [1, 2, 3, 5, 8, 13, 21] {
            assert_eq!(expected, fib.next().unwrap())
        }
    }
}
