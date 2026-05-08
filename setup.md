# Project Euler in Rust — Project Setup

Create a new Cargo workspace for solving Project Euler problems in Rust. Use the exact structure and
file contents below.

## Structure

```
project-euler/
├── Cargo.toml           # workspace manifest
├── .gitignore
├── shared/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── primes.rs
│       ├── math.rs
│       └── digits.rs
└── problems/
    ├── Cargo.toml
    └── src/
        └── bin/
            └── p0001.rs
```

Rationale: a single `problems` crate with binaries under `src/bin/` means adding a new problem is
just dropping a new `.rs` file — no manifest edits, no workspace member updates. The `shared` crate
holds reusable helpers (primes, gcd, digit manipulation, etc.). Adding a new problem with
`src/bin/pNNNN.rs` is automatically picked up by Cargo.

## Files

### `Cargo.toml` (workspace root)

```toml
[workspace]
resolver = "2"
members = ["shared", "problems"]

[workspace.package]
edition = "2021"

[profile.release]
lto = "fat"
codegen-units = 1
```

### `.gitignore`

```
/target
Cargo.lock
```

(Keep `Cargo.lock` if you want reproducible builds across machines; remove that line in that case.)

### `shared/Cargo.toml`

```toml
[package]
name = "shared"
version = "0.1.0"
edition.workspace = true
```

### `shared/src/lib.rs`

```rust
pub mod primes;
pub mod math;
pub mod digits;
```

### `shared/src/primes.rs`

```rust
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

/// Returns all primes up to and including `limit`.
pub fn primes_up_to(limit: usize) -> Vec<usize> {
    sieve(limit)
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| if p { Some(i) } else { None })
        .collect()
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
```

### `shared/src/math.rs`

```rust
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { a / gcd(a, b) * b }
}
```

### `shared/src/digits.rs`

```rust
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
```

### `problems/Cargo.toml`

```toml
[package]
name = "problems"
version = "0.1.0"
edition.workspace = true

[dependencies]
shared = { path = "../shared" }
```

### `problems/src/bin/p0001.rs`

```rust
// Problem 1: Sum of all multiples of 3 or 5 below 1000.
fn main() {
    let sum: u64 = (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
    println!("{sum}");
}
```

## Usage

Run a specific problem:

```
cargo run --release --bin p0001
```

Add a new problem by creating `problems/src/bin/pNNNN.rs` — no other changes needed.

Build everything:

```
cargo build --release
```

Run tests (for the `shared` crate):

```
cargo test
```
