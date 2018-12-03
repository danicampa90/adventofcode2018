extern crate libutils;
mod parser;
mod field;
use libutils::read_file_lines_in_chan;
use std::collections::HashSet;
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::ops::IndexMut;


fn first_star(file_rx: Receiver<String>) -> i32 {
    let mut field = field::Field::new(1000,1000);
    loop{
        match file_rx.recv() {
            Ok(line) => { 
                let reservation = parser::ParsedLine::from_line(&line);
                field.add_reservation(reservation);
            },
            Err(_) => break
        }
    }
    return field.count_conflicts();
}

fn second_star() -> String {
    String::new()
}

fn main() {
    let file_rx = read_file_lines_in_chan("input.txt").expect("Cannot open the file");
    let conflicts = first_star(file_rx);
    println!("Got {} conflicts.", conflicts);
    let result = second_star();
    println!("result is {}", result );
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser_works() {
        let parsed = parser::ParsedLine::from_line("#12 @ 34,56: 7x890");
        assert_eq!(parsed.id, 12);
        assert_eq!(parsed.start_x, 34);
        assert_eq!(parsed.start_y, 56);
        assert_eq!(parsed.size_x, 7);
        assert_eq!(parsed.size_y, 890);
    }
    
}