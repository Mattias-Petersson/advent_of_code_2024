use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> Result<u32, Box<dyn Error>> {
    let (first, second) = read_input()?;
    let distance = get_distance(first, second);
    Ok(distance)
}

fn read_input() -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();

    let path: &'static str = "src/day1/input.txt";
    let input: File =
        File::open(path).map_err(|err| format!("Failed to open {}: {}", path, err))?;
    let buffer: BufReader<File> = BufReader::new(input);

    for line in buffer.lines() {
        let line = line?;
        let mut values = line.split_whitespace();

        if let (Some(x), Some(y)) = (values.next(), values.next()) {
            first.push(x.parse().unwrap());
            second.push(y.parse().unwrap());
        }
    }
    Ok((first, second))
}

fn get_distance(mut first: Vec<u32>, mut second: Vec<u32>) -> u32 {
    first.sort();
    second.sort();

    let mut distance = 0;
    let len = first.len();

    for i in 0..len {
        distance += first[i].abs_diff(second[i]);
    }
    distance
}
