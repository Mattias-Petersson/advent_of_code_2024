use advent_of_code_2024::read_from_file;
use regex::Regex;
use std::{error::Error, io::Read};

pub fn exercise() {
    match sum_pairs() {
        Ok(num) => {
            println!("Sum of all muls {}", num);
        }
        Err(e) => {
            println!("Err {:?}", e);
        }
    }
}

fn sum_pairs() -> Result<i32, Box<dyn Error>> {
    let mut buffer = read_from_file("src/day3/input.txt")?;
    let mut buff_to_str = String::new();
    buffer.read_to_string(&mut buff_to_str)?;
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let summation_of_pairs: Result<i32, Box<dyn Error>> =
        re.captures_iter(&buff_to_str)
            .try_fold(0, |accum, capture| {
                // Attempt to parse a capture group i to a i32.
                let parse_int = |i| -> Result<i32, Box<dyn Error>> {
                    Ok(capture
                        .get(i)
                        .ok_or_else(|| format!("Missing capture group at index {}", i))?
                        .as_str()
                        .parse::<i32>()?)
                };
                // First value is the entire match, second is the first capture group (first integer),
                // third the second capture group (second integer)
                let n = parse_int(1)?;
                let m = parse_int(2)?;
                Ok(accum + n * m)
            });
    Ok(summation_of_pairs?)
}
