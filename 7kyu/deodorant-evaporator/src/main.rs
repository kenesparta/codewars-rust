fn evaporator(content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let mut content_mutable = content;
    let mut day: i32 = 0;
    while content_mutable / content * 100.0 >= threshold as f64 {
        content_mutable *= 1.0 - evap_per_day as f64 / 100.0;
        day += 1;
    }
    return day;
    // Math solution
    // (f64::from(threshold) / 100.0)
    //     .log(1.0 - f64::from(evap_per_day) / 100.0)
    //     .ceil() as i32
}

fn main() {
    println!("{}", evaporator(10.0, 10, 5));
}
