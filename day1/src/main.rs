use std::fs::File;
use std::io::{self, BufRead};
use std::os::linux::raw;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_first_int(string: &str) -> char {

    for character in string.chars() {
        match character.is_digit(10) {
            true => return character,
            false => {}
        }
    }
    return '0';
}

fn get_last_int(string: &str) -> char {
    for character in string.chars().rev() {
        match character.is_digit(10) {
            true => return character,
            false => {}
        }
    }
    return '0';
}

fn main() { 
    let mut total = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(raw_line) = line {
                
                let mut number = String::new();
                number.push(get_first_int(&raw_line));
                number.push(get_last_int(&raw_line));

                total += number.parse::<u32>().unwrap();

            }
        }
    }
    println!("{:}", total);
}
