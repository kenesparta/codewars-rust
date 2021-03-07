// use itertools::Itertools;

fn sequence(x: u8) -> Vec<u8> {
    let mut stuff_str = Vec::new();
    stuff_str = (1..x + 1).into_iter().map(|i| i.to_string()).collect();
    stuff_str.sort();
    return stuff_str
        .into_iter()
        .map(|s| s.parse().expect("parse error"))
        .collect();
}

// fn sequence_better(x: i32) -> Vec<i32> {
//     (1..=x)
//         .map(|n| n.to_string())
//         .sorted()
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect()
// }

fn main() {
    assert_eq!(
        sequence(16),
        [1, 10, 11, 12, 13, 14, 15, 16, 2, 3, 4, 5, 6, 7, 8, 9],
        "sequence(16)",
    );
    assert_eq!(sequence(9), [1, 2, 3, 4, 5, 6, 7, 8, 9], "sequence(9)",);
}
