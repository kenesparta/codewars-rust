// My solution
fn descending_order(x: u64) -> u64 {
    let mut digits = Vec::new();
    let mut n = x;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.sort();
    digits.reverse();

    // Convert from array to number
    let base: u64 = 10;
    let mut size_digits = digits.len() as u32;
    let mut result: u64 = 0;
    for d in &digits {
        size_digits -= 1;
        result += base.pow(size_digits) * d;
    }
    result
}

// Other solution
use std::iter::FromIterator;
fn descending_order_better(x: u64) -> u64 {
    let mut result = x.to_string().chars().collect::<Vec<char>>();
    result.sort_by(|a, b| b.cmp(a));
    String::from_iter(result).parse::<u64>().unwrap()
}

fn main() {
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
