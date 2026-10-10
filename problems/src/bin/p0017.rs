const ONE_THROUGH_NINE: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const TEN_THROUGH_NINETEEN: &[&str] = &[
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const UNIQUE: &[&str] = &["onethousand"];
const TENS: &[&str] = &[
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const HUNDRED: &str = "hundred";

fn main() {
    let otn_count: usize = ONE_THROUGH_NINE.iter().map(|n| n.len()).sum();
    let ttn_count: usize = TEN_THROUGH_NINETEEN.iter().map(|n| n.len()).sum();
    let tens_count: usize = TENS
        .iter()
        .map(|n| n.len() + n.len() * ONE_THROUGH_NINE.len() + otn_count)
        .sum();
    let sub_hundred = otn_count + ttn_count + tens_count;
    let hundreds_count: usize = ONE_THROUGH_NINE
        .iter()
        .map(|n| n.len() + HUNDRED.len() + (n.len() + HUNDRED.len() + 3) * 99 + sub_hundred)
        .sum();

    let result: usize =
        sub_hundred + hundreds_count + UNIQUE.iter().map(|n| n.len()).sum::<usize>();
    println!("{result}");
}
