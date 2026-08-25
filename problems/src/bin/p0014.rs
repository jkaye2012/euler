use shared::math::CollatzSequence;

fn main() {
    let mut longest = 0;
    let mut result = 0;
    for start in 500001..1000000 {
        let seq = CollatzSequence::new(start);
        let length = seq.count();
        if length >= longest {
            longest = length;
            result = start;
        }
    }
    println!("{result}, {longest}");
}
