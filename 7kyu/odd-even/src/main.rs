fn odd_or_even(numbers: Vec<i32>) -> String {
    let mut sum: i32;
    sum = 0;
    for element in numbers {
        sum += element;
    }
    if sum % 2 == 0 {
        return "even".to_string();
    }
    return "odd".to_string();
}

fn odd_or_even_better(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() % 2 == 0 {
        true => "even".to_string(),
        false => "odd".to_string(),
    }
}

fn main() {
    assert_eq!(odd_or_even(vec![0, 1, 4]), "odd");
    assert_eq!(odd_or_even_better(vec![0, 1, 4]), "odd");
}
