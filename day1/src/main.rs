extern crate libutils;
use libutils::read_file_lines_in_chan;
use std::collections::HashSet;
use std::thread;
use std::sync::mpsc::channel;

fn second_part(numbers: Vec<i32>) -> i32 {
    
    let mut seen_twice_dict = HashSet::new();
    let mut sum = 0;
    for number in numbers.into_iter().cycle() {
        
        if !seen_twice_dict.insert(sum) {
            break;
        }
        sum += number;
    }
    return sum;
}



fn main() {
    println!("Hello, world!");
    let file_rx = read_file_lines_in_chan("input.txt").expect("Cannot open the file");

    let mut sum:i32 = 0;

    let mut numbers : Vec<i32> = Vec::new();

    loop {
        match file_rx.recv() {
            Ok(r) => {
                let number = r.parse::<i32>().expect("Not a number!");
                numbers.push(number);
                sum+= number;
            },
            Err(_) => break
        }
    }
    println!("Resulting frequency: {}", sum);
    let repeats_at = second_part(numbers);
    println!("It repeats at value {}", repeats_at);
}


#[cfg(test)]
mod tests {
    use super::second_part;
    #[test]
    fn second_start_test_1() {
        assert_eq!(second_part(vec!(1, -1)), 0);
    }
    #[test]
    fn second_start_test_2() {
        assert_eq!(second_part(vec!(3, 3, 4, -2, -4)), 10);
        }
    #[test]
    fn second_start_test_3() {
        assert_eq!(second_part(vec!(-6, 3, 8, 5, -6)), 5);
        }
    #[test]
    fn second_start_test_4() {
        assert_eq!(second_part(vec!(7, 7, -2, -7, -4)), 14);
    }
}