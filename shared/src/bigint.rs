use std::{
    fmt::{self, Display},
    iter::repeat,
    ops::{Add, Mul, MulAssign},
    slice::Iter,
    str::FromStr,
};

/// Arbitrary length unsigned integer.
#[derive(Clone)]
pub struct BigUint {
    digits: Vec<u8>,
}

impl BigUint {
    /// Creates a new `BigUint` initialized to 0.
    pub fn new() -> Self {
        Self { digits: vec![0] }
    }

    /// Iterate the individual digits of the integer.
    pub fn iter(&self) -> Iter<'_, u8> {
        self.digits.iter()
    }

    fn mul_scalar(&self, n: u8, places: usize) -> Self {
        let mut digits = Vec::with_capacity(self.digits.len());
        if n == 0 {
            return Self { digits };
        }
        for _ in 0..places {
            digits.push(0);
        }

        let mut carry = 0;
        for digit in self.digits.iter() {
            let place = digit * n + carry;
            digits.push(place % 10);
            carry = place / 10;
        }

        if carry > 0 {
            digits.push(carry);
        }

        Self { digits }
    }
}

impl From<u32> for BigUint {
    fn from(value: u32) -> Self {
        let mut v = value;
        let mut digits: Vec<u8> = Vec::new();
        while v > 0 {
            digits.push(v as u8 % 10);
            v /= 10;
        }
        if digits.is_empty() {
            digits.push(0);
        }

        Self { digits }
    }
}

impl Default for BigUint {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for BigUint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|c| !c.is_ascii_digit()) {
            return Err("cannot parse non-ascii digit into BigUint");
        }
        let digits = s.as_bytes().iter().map(|b| b - 48).rev().collect();
        Ok(Self { digits })
    }
}

impl Display for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

impl Add for BigUint {
    type Output = BigUint;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs_extend = rhs.digits.len().saturating_sub(self.digits.len());
        let rhs_extend = self.digits.len().saturating_sub(rhs.digits.len());
        let lhs_iter = self.digits.iter().chain(repeat(&0u8).take(lhs_extend));
        let rhs_iter = rhs.digits.iter().chain(repeat(&0u8).take(rhs_extend));

        let mut digits = Vec::new();
        let mut carry = 0;
        for (lhsd, rhsd) in lhs_iter.zip(rhs_iter) {
            let place = lhsd + rhsd + carry;
            digits.push(place % 10);
            carry = place / 10;
        }

        if carry > 0 {
            digits.push(carry);
        }

        Self { digits }
    }
}

impl Mul<&BigUint> for &BigUint {
    type Output = BigUint;

    fn mul(self, rhs: &BigUint) -> Self::Output {
        let (short, long) = if self.digits.len() <= rhs.digits.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };
        let mut result = BigUint::new();
        for (place, digit) in short.digits.iter().enumerate() {
            result = result + long.mul_scalar(*digit, place);
        }
        result
    }
}

impl Mul for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: &BigUint) -> Self::Output {
        &self * rhs
    }
}

impl Mul<BigUint> for &BigUint {
    type Output = BigUint;

    fn mul(self, rhs: BigUint) -> Self::Output {
        self * &rhs
    }
}

impl MulAssign<&BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: &BigUint) {
        let product = rhs * &*self;
        self.digits = product.digits;
    }
}
