use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2024::read_from_file;

const _MATCH_PHRASES: [[char; 4]; 2] = [['X', 'M', 'A', 'S'], ['S', 'A', 'M', 'X']];

pub fn exercise() {
    let result = || -> Result<usize, Box<dyn Error>> {
        let _input = get_input()?;
        let hori = read_horizontal(&_input);
        println!("{hori}");
        let vert = read_vertical(&_input);
        Ok(hori + vert)
    };
    match result() {
        Ok(val) => println!("Occurrences of \"XMAS\" & \"SAMX\" in input: {val}"),
        Err(_) => todo!(),
    }
}

/// Reads in the input and returns a Vec<Vec<char>>
fn get_input() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let buffer: BufReader<File> = read_from_file("src/day4/input.txt")?;
    buffer
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect()
}

fn count_matches(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .flat_map(|row| row.windows(4))
        .filter(|window| {
            _MATCH_PHRASES
                .iter()
                .any(|match_pattern| match_pattern == window)
        })
        .count()
}

fn read_horizontal(input: &Vec<Vec<char>>) -> usize {
    count_matches(&input)
}

fn read_vertical(input: &Vec<Vec<char>>) -> usize {
    let col_len = input[0].len();
    let transposed = (0..col_len)
        .map(|col| input.iter().map(|row| row[col]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    count_matches(&transposed)
}

fn read_diagonal(input: &Vec<Vec<char>>) -> usize {
    todo!();
}
