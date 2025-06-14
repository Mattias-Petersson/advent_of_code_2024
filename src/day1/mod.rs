use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn exercise() -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let (mut first, mut second) = read_input()?;
    let distance = part1(&mut first, &mut second)?;
    let similarity = part2(&mut first, &mut second);
    Ok((distance, similarity))
}

fn part1(first: &mut Vec<u32>, second: &mut Vec<u32>) -> Result<u32, Box<dyn Error>> {
    let distance = get_distance(first, second);
    Ok(distance)
}

pub fn part2(first: &mut Vec<u32>, second: &mut Vec<u32>) -> u32 {
    let mut freq = HashMap::new();
    for num in second.iter() {
        *freq.entry(num).or_insert(0) += 1;
    }
    let similarity = first
        .iter()
        .map(|&num| num * freq.get(&num).copied().unwrap_or(0))
        .sum();
    similarity
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
            first.push(x.parse()?);
            second.push(y.parse()?);
        }
    }
    Ok((first, second))
}

fn get_distance(first: &mut Vec<u32>, second: &mut Vec<u32>) -> u32 {
    first.sort();
    second.sort();

    let mut distance = 0;
    let len = first.len();

    for i in 0..len {
        distance += first[i].abs_diff(second[i]);
    }
    distance
}
