fn elevator_distance(floors: &[i16]) -> i16 {
    let mut result: i16 = 0;
    let mut idx: usize = 0;
    while idx < floors.len() - 1 {
        result += (floors[idx] - floors[idx + 1]).abs();
        idx += 1;
    }
    return result;
}

// better option
// fn elevator_distance_better(floors: &[i16]) -> i16 {
//     floors.windows(2).map(|s| (s[0] - s[1]).abs()).sum()
// }

fn main() {
    assert_eq!(elevator_distance(&[5, 2, 8]), 9);
    assert_eq!(elevator_distance(&[1, 2, 3]), 2);
    assert_eq!(elevator_distance(&[7, 1, 7, 1]), 18);
}
