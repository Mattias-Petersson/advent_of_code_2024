use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_from_file(path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let input: File =
        File::open(path).map_err(|err| format!("Failed to open {}: {}", path, err))?;
    let buffer: BufReader<File> = BufReader::new(input);
    Ok(buffer)
}

/// Reads in the input and returns a Vec<Vec<char>>
pub fn get_input_to_char_array(path: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let buffer: BufReader<File> = read_from_file(path)?;
    buffer
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect()
}
