use advent_of_code_2024::read_from_file;
use std::error::Error;
use std::io::BufRead;

pub fn exercise() {
    match count_safe_levels() {
        Ok(num_safe) => {
            println!("There are {:?} safe lines", num_safe)
        }
        Err(e) => {
            eprintln!("Error {e}")
        }
    }
}

fn count_safe_levels() -> Result<i32, Box<dyn Error>> {
    let buffer = read_from_file("src/day2/input.txt")?;
    let mut num_safe = 0;
    for line in buffer.lines() {
        let line = line?;
        let values = line
            .split_whitespace()
            .map(|i| i.parse::<i32>())
            .collect::<Result<_, _>>()?;
        num_safe += is_safe_with_pb(&values) as i32;
    }
    Ok(num_safe)
}

fn is_safe_with_pb(list: &Vec<i32>) -> bool {
    match is_safe(&list) {
        true => true,
        false => {
            for i in 0..list.len() {
                let temp: Vec<i32> = list[..i].iter().chain(&list[i + 1..]).copied().collect();
                if is_safe(&temp) {
                    return true;
                }
            }
            false
        }
    }
}

enum Direction {
    Increase,
    Decrease,
}

fn is_safe(list: &Vec<i32>) -> bool {
    if list.len() < 2 {
        return true;
    }

    let direction: Direction = if list[1] - list[0] > 0 {
        Direction::Increase
    } else {
        Direction::Decrease
    };

    for i in 1..list.len() {
        let difference = list[i] - list[i - 1];
        if difference == 0 || difference.abs() > 3 {
            return false;
        }
        match direction {
            Direction::Increase if difference < 0 => return false,
            Direction::Decrease if difference > 0 => return false,
            _ => {}
        }
    }
    true
}
