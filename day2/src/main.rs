extern crate libutils;
mod string_count;
use libutils::read_file_lines_in_chan;
use std::collections::HashSet;
use std::thread;
use std::sync::mpsc::channel;
use std::ops::IndexMut;


fn first_star() {
    let file_rx = read_file_lines_in_chan("input.txt").expect("Cannot open the file");
    
    let (hashes_tx, hashes_rx) = channel();
    thread::spawn(move || {
        loop {
            match (file_rx.recv()) {
                Ok(val) => hashes_tx.send(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies(&val))),
                Err(_) => break
            }.unwrap();
        }
    });
    
    let mut count_2 = 0;
    let mut count_3 = 0;
    loop {
        match (hashes_rx.recv()) {
            Ok((true,false)) => count_2+= 1,
            Ok((false,true)) => count_3+= 1,
            Ok((true,true)) => {count_2+=1; count_3+=1},
            Ok((false,false)) => (),
            Err(_) => break
        }
    }
    println!("Hash is {}", count_2 * count_3);
}

// See second_star() for description of what this does.
struct StrCombinationIterator<'a> {
    input: &'a str,
    index : i32
}
impl<'a> StrCombinationIterator<'a>  {
    fn new(input:&'a str) -> StrCombinationIterator<'a> {
        StrCombinationIterator {
            input:input, 
            index : input.len() as i32
        }
    }
}
impl<'a> Iterator for StrCombinationIterator<'a>  {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.index -= 1;
        return if self.index >= 0 {    
            let mut retval = String::new();
            for (c_idx, c) in self.input.char_indices() {
                retval.push(if c_idx == self.index as usize {'.'} else {c});
            }
            Some(retval)
        } else {
            None
        }

    }
}
fn second_star() -> String {
    let file_rx = read_file_lines_in_chan("input.txt").expect("Cannot open the file");
    // hashset of all combinations of the input strings + 1 character replaced with a dot.
    // example: given "abcd", we will add the values ".bcd", "a.cd", "ab.d", "abc." to the hahset.
    // if a string differs only by 1 character then we will have a duplicate in the hashet.
    // this is more efficient than iterating through all the strings (that has a O(n^2) complexity, 
    // while this is O(n*k) complexity (where k is the number of characters in each word))
    let mut hash = HashSet::new();

    loop {
        match (file_rx.recv()) {
            Ok(val) => for combination in StrCombinationIterator::new(&val) {
                if !hash.insert(combination.clone()) { return combination; }
            }
            Err(_) => return String::new()
        }
    }
}

fn main() {
    first_star();
    let result = second_star();
    println!("result is {}", result );
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_letter_count_works() {
        assert_eq!(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies("abcdef")), (false, false));
        assert_eq!(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies("bababc")), (true, true));
        assert_eq!(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies("abbcde")), (true, false));
        assert_eq!(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies("abcccd")), (false, true));
        assert_eq!(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies("aabcdd")), (true, false));
        assert_eq!(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies("abcdee")), (true, false));
        assert_eq!(string_count::contains_2_or_3_duplicate_letters(string_count::letter_frequencies("ababab")), (false, true));
    }
    
    #[test]
    fn combination_it_works() {
        let result : Vec<String>= StrCombinationIterator::new("abcde").collect();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "abcd.");
        assert_eq!(result[1], "abc.e");
        assert_eq!(result[2], "ab.de");
        assert_eq!(result[3], "a.cde");
        assert_eq!(result[4], ".bcde");
    }
}