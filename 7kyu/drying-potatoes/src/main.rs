fn potatoes(p0: i64, w0: i64, p1: i64) -> i64 {
    (w0 as f64 * (100.0 - p0 as f64) / (100.0 - p1 as f64)) as i64
}

fn main() {
    //    potatoes(82, 127, 80);
    assert_eq!(potatoes(99, 100, 98), 50);
    assert_eq!(potatoes(82, 127, 80), 114);
}
