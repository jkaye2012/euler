fn main() {
    // There exists a more elegant solution that directly computes b
    // by substitution of c = 1000 - a - b, but with n = 1000 it's
    // slower than brute force
    let mut result = 0u32;
    'outer: for a in 1..1000u32 {
        for b in (a + 1)..1000u32 {
            let c = 1000 - a - b;
            if a + b + c != 1000 {
                continue;
            }
            if a.pow(2) + b.pow(2) == c.pow(2) {
                result = a * b * c;
                break 'outer;
            }
        }
    }

    println!("{result}");
}
