use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




fn main() {
    let BLUES = 14;
    let GREENS = 13;
    let REDS =  12;

    let game_id_regex = Regex::new(r"[Gg]ame (?<id>\d+):.*").unwrap();
    let blues_regex = Regex::new(r"(?<num>\d+) blue").unwrap();
    let greens_regex = Regex::new(r"(?<num>\d+) green").unwrap();
    let reds_regex = Regex::new(r"(?<num>\d+) red").unwrap();

    let mut total = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line  in lines {
            if let Ok(raw_line) = line {
                let game_id: u32 = (&game_id_regex.captures(&raw_line).unwrap()["id"]).parse().unwrap();
                let blue_triggered = blues_regex.captures_iter(&raw_line).map(|c| c.name("num").unwrap().as_str()).any(|x| x.parse::<i32>().unwrap() > BLUES);
                let green_triggered = greens_regex.captures_iter(&raw_line).map(|c| c.name("num").unwrap().as_str()).any(|x| x.parse::<i32>().unwrap() > GREENS);
                let red_triggered = reds_regex.captures_iter(&raw_line).map(|c| c.name("num").unwrap().as_str()).any(|x| x.parse::<i32>().unwrap() > REDS);

                if !(blue_triggered || green_triggered || red_triggered) {
                    total += game_id;
                }

            }   
        }
        println!("Part 1: {:}", total);
    }
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total = 0;
        for line  in lines {
            if let Ok(raw_line) = line {
                let game_id: u32 = (&game_id_regex.captures(&raw_line).unwrap()["id"]).parse().unwrap();
                let num_blues = blues_regex.captures_iter(&raw_line).map(|c| c.name("num").unwrap().as_str().parse::<u32>().unwrap()).max().unwrap();
                let num_greens = greens_regex.captures_iter(&raw_line).map(|c| c.name("num").unwrap().as_str().parse::<u32>().unwrap()).max().unwrap();
                let num_reds = reds_regex.captures_iter(&raw_line).map(|c| c.name("num").unwrap().as_str().parse::<u32>().unwrap()).max().unwrap();

                let power: u32 = num_blues*num_greens*num_reds;
                
                total += power;


            }   
        }
        println!("Part 2: {:}", total);
    }
}
