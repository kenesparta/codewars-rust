fn seven(n: i64) -> (i64, i32) {
    // let div = n % 10;
    let mut temporal_number = n;
    let mut number_steps: i32;
    number_steps = 0;
    while temporal_number >= 100 {
        temporal_number = temporal_number / 10 - 2 * (temporal_number % 10);
        number_steps += 1;
    }
    (temporal_number, number_steps)
}

fn main() {
    println!("{:?}", seven(477557101));
}
