use advent_of_code_2024::read_from_file;
use regex::Regex;
use std::error::Error;
use std::io::Read;

pub fn exercise() {
    let result = || -> Result<i32, Box<dyn Error>> {
        let mut buffer = read_from_file("src/day3/input.txt")?;
        let mut buff_to_str = String::new();
        buffer.read_to_string(&mut buff_to_str)?;
        let filtered_input = handle_instructions(&buff_to_str)?;
        sum_pairs(&filtered_input)
    };
    match result() {
        Ok(num) => println!("Sum of all muls {num}"),
        Err(e) => eprintln!("Err {e}"),
    }
}

fn handle_instructions(input: &str) -> Result<String, Box<dyn Error>> {
    let re: Regex = Regex::new(r"(do\(\)|don't\(\))")?;
    let str_parts: Vec<_> = re.split(input).collect();
    let instructions: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();

    // str_parts is 64 long, instructions is 63. In the assignment it is said that it starts in an enabled state.
    let mut result = String::from(str_parts[0]);
    for pairs in str_parts.iter().skip(1).zip(instructions) {
        let (text, instr) = pairs;
        match instr {
            "do()" => result.push_str(text),
            _ => (),
        }
    }
    Ok(result)
}

fn sum_pairs(input: &str) -> Result<i32, Box<dyn Error>> {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let parse_int = |capture: &regex::Captures, i| -> Result<i32, Box<dyn Error>> {
        Ok(capture
            .get(i)
            .ok_or_else(|| format!("Missing capture group at index {}", i))?
            .as_str()
            .parse::<i32>()?)
    };

    let summation_of_pairs: Result<i32, Box<dyn Error>> =
        re.captures_iter(input).try_fold(0, |accum, capture| {
            // Attempt to parse a capture group i to a i32.

            // First value is the entire match, second is the first capture group (first integer),
            // third the second capture group (second integer)
            let n = parse_int(&capture, 1)?;
            let m = parse_int(&capture, 2)?;
            Ok(accum + n * m)
        });
    Ok(summation_of_pairs?)
}
