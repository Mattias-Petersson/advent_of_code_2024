use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2024::read_from_file;

pub fn exercise() {
    let result = || -> Result<i32, Box<dyn Error>> {
        let _input = get_input()?;
        println!("{:?}", _input);
        Ok(read_vertical(&_input)? + read_horizontal(&_input)? + read_diagonal(&_input)?)
    };
    match result() {
        Ok(val) => println!("{val}"),
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

fn count_line(line: Vec<char>) {
    const _MATCH_PHRASE: [char; 4] = ['X', 'M', 'A', 'S'];
    const _MATCH_PHRASE_REV: [char; 4] = ['S', 'A', 'M', 'X'];
}

fn read_vertical(input: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>> {
    todo!();
}

fn read_horizontal(input: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>> {
    todo!();
}

fn read_diagonal(input: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>> {
    todo!();
}
