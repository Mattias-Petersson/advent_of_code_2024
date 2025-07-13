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
        let horizontal_count = read_horizontal(&_input);
        let vertical_count = read_vertical(&_input);
        let diagonal_count = read_diagonal(&_input);
        Ok(horizontal_count + vertical_count + diagonal_count)
    };
    match result() {
        Ok(val) => println!("Occurrences of \"XMAS\" & \"SAMX\" in input: {val}"),
        Err(_) => todo!(),
    }
}

/// Reads in the input and returns a Vec<Vec<char>>
fn get_input() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let buffer: BufReader<File> = read_from_file("src/week1/day4/input.txt")?;
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
    let mut diagonals: Vec<Vec<char>> = Vec::new();
    acc_diagonals(input, &mut diagonals);
    count_matches(&diagonals)
}

fn acc_diagonals(input: &Vec<Vec<char>>, acc_diags: &mut Vec<Vec<char>>) {
    for col_start in 0..input[0].len() {
        let left_to_right = get_one_diagonal(input, 0, col_start, 1);
        let right_to_left = get_one_diagonal(input, 0, col_start, -1);

        acc_diags.push(left_to_right);
        acc_diags.push(right_to_left);
    }
    for row_start in 1..input.len() {
        let left_to_right = get_one_diagonal(input, row_start, 0, 1);
        let right_to_left = get_one_diagonal(input, row_start, input[0].len() - 1, -1);

        acc_diags.push(left_to_right);
        acc_diags.push(right_to_left);
    }
}

/// col_delta is positive when going left-to-right, and negative when going right-to-left.
fn get_one_diagonal(
    input: &Vec<Vec<char>>,
    row_start: usize,
    col_start: usize,
    col_delta: isize,
) -> Vec<char> {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut diag: Vec<_> = Vec::new();

    let mut curr_row = row_start;
    let mut curr_col = col_start as isize;

    while curr_row < num_rows && (curr_col as usize) < num_cols {
        diag.push(input[curr_row][curr_col as usize]);
        curr_row += 1;
        if curr_col == 0 && col_delta < 0 {
            break;
        }
        curr_col += col_delta;
    }
    diag
}
