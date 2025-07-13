use advent_of_code_2024::read_from_file;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

pub fn exercise() {
    match run_both_parts() {
        Ok((distance, similarity)) => {
            println!("Distance is {:?}", distance);
            println!("Similarity is {:?}", similarity);
        }
        Err(e) => println!("Failed with error {e}"),
    }
}

fn run_both_parts() -> Result<(u32, u32), Box<dyn Error>> {
    let (mut first, mut second) = read_input()?;
    let distance = part1(&mut first, &mut second)?;
    let similarity = part2(&first, &second);
    Ok((distance, similarity))
}

fn part1(first: &mut Vec<u32>, second: &mut Vec<u32>) -> Result<u32, Box<dyn Error>> {
    let distance = get_distance(first, second);
    Ok(distance)
}

pub fn part2(first: &[u32], second: &[u32]) -> u32 {
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

    let buffer = read_from_file("src/week1/day1/input.txt")?;

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
