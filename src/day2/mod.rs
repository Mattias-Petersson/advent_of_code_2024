use advent_of_code_2024::read_from_file;
use std::error::Error;
use std::io::BufRead;

pub fn exercise() {
    match part1() {
        Ok(count_safe) => {
            println!("There are {:?} safe lines", count_safe)
        }
        Err(e) => {
            println!("Error {e}")
        }
    }
}

fn part1() -> Result<i32, Box<dyn Error>> {
    let buffer = read_from_file("src/day2/input.txt")?;
    let mut count_safe = 0;
    for line in buffer.lines() {
        let line = line?;
        let values = line
            .split_whitespace()
            .map(|i| i.parse::<i32>())
            .collect::<Result<_, _>>()?;
        count_safe += is_safe(&values) as i32;
    }
    Ok(count_safe)
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
            Direction::Increase => {
                if difference < 0 {
                    return false;
                }
            }
            Direction::Decrease => {
                if difference > 0 {
                    return false;
                }
            }
        }
    }
    true
}
