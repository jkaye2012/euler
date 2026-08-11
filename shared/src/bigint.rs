use std::{
    fmt::{self, Display},
    iter::repeat,
    ops::Add,
    str::FromStr,
};

/// Arbitrary length unsigned integer.
pub struct BigUint {
    digits: Vec<u8>,
}

impl BigUint {
    /// Creates a new `BigUint` initialized to 0.
    pub fn new() -> Self {
        Self { digits: vec![0] }
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
