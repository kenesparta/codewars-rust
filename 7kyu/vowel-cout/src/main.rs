fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    let list_vowels = vec!['a', 'e', 'i', 'o', 'u'];
    for c in string.chars() {
        if list_vowels.contains(&c) {
            vowels_count += 1;
        }
    }
    vowels_count
}

fn main() {
    assert_eq!(get_count("abracadabra"), 5);
}
