fn lowest_divisible_by_all_up_to(n: u64) -> u64 {
    'outer: for i in std::iter::successors(Some(n), |&prev| Some(prev + n)) {
        for j in 1..n {
            if i % j != 0 {
                continue 'outer;
            }
        }
        return i;
    }
    unreachable!();
}
fn main() {
    let result = lowest_divisible_by_all_up_to(20);
    println!("{result}");
}
