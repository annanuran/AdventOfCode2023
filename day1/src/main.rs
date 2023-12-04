use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

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

fn find_overlapping_matches(pattern: &str, text: &str) -> Vec<char>{
    let mut numbers = Vec::new();

    let regex = Regex::new(pattern).expect("Failed to create regex pattern");

    let mut start = 0;

    while start < text.len() {
        if let Some(captures) = regex.captures(&text[start..]) {
            let start_match = captures.get(0).unwrap().start();
            let end_match = captures.get(0).unwrap().end();

            numbers.push(match &text[start + start_match..start + end_match] {
                "one" => '1',
                "two" => '2',
                "three" => '3',
                "four" => '4',
                "five" => '5',
                "six" => '6',
                "seven" => '7',
                "eight" => '8',
                "nine" => '9',
                x => if x.len() == 1 {x.chars().nth(0).unwrap()} else {'0'}
            });
            start += start_match + 1;
        } else {
            start += 1;
        }
    }
    return numbers;
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
    println!("Part 1: {:}", total);
    // part 2 --------- 
    let mut total = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(raw_line) = line {
                // get all numbers in line, in order, as digit characters. Needs to be collected as a string for lifetime reasons 🦀
                let numbers_in_str = find_overlapping_matches(r"(one|two|three|four|five|six|seven|eight|nine|\d)", &raw_line).iter().cloned().collect::<String>();

                // first and last numbers in line make the calibration value, so just take the first and last characters of the string
                let calibration_num = numbers_in_str.chars().take(1).collect::<String>()
                + &numbers_in_str.chars().last().unwrap().to_string();
                
                total += calibration_num.parse::<u32>().unwrap();

            }
        }
    }
    println!("Part 2: {:}", total);
}
