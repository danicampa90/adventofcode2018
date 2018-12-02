use std::collections::HashMap;

type CharFrequencies = HashMap<char, i32>;

pub fn letter_frequencies(text: &str) -> CharFrequencies {
    let mut result : CharFrequencies = HashMap::new();
    for letter in text.chars() {
        *result.entry(letter).or_insert(0) += 1;
    }
    return result;
}
pub fn contains_2_or_3_duplicate_letters(text: CharFrequencies) -> (bool, bool) {
    let mut contains_2 = false;
    let mut contains_3 = false;
    for frequency in text.values() {
        match *frequency {
            2 => contains_2 = true,
            3 => contains_3 = true,
            _ => ()
        }
    }
    return (contains_2, contains_3)
}