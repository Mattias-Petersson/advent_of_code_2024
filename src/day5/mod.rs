use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2024::read_from_file;

type Rules = Vec<(u32, u32)>;
type Pages = Vec<Vec<u32>>;

pub fn exercise() {
    let res = || -> Result<u32, Box<dyn Error>> {
        let input = get_input()?;
        let vectors = find_all_valid_books(&input)?;
        let n = calculate_from_valid(vectors);
        Ok(n)
    };
    match res() {
        Ok(n) => println!("Success. {n}"),
        Err(e) => eprintln!("Error. {e}"),
    }
}

fn get_input() -> Result<(Rules, Pages), Box<dyn Error>> {
    let buffer: BufReader<File> = read_from_file("src/day5/input.txt")?;
    let mut is_rule = true; // start by reading rules.
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut pages: Vec<Vec<u32>> = Vec::new();
    for line in buffer.lines() {
        let line = line?;

        if line.is_empty() {
            is_rule = false;
            continue;
        }
        match is_rule {
            true => {
                let mut nums = line.split('|');
                if let (Some(x), Some(y)) = (nums.next(), nums.next()) {
                    rules.push((x.parse()?, y.parse()?));
                }
            }
            false => {
                let vec: Vec<u32> = line
                    .split(',')
                    .map(|v| v.trim().parse())
                    .collect::<Result<_, _>>()?;
                pages.push(vec);
            }
        }
    }
    Ok((rules, pages))
}

fn find_all_valid_books(input: &(Rules, Pages)) -> Result<Vec<&Vec<u32>>, Box<dyn Error>> {
    let (rules, pages) = input;
    let mut valid_pages = Vec::new();

    for vec in pages {
        let mut is_valid = true;
        for rule in rules {
            if let (Some(first), Some(second)) = (
                vec.iter().position(|v| v == &rule.0),
                vec.iter().position(|v| v == &rule.1),
            ) {
                if first > second {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            valid_pages.push(vec);
        }
    }

    Ok(valid_pages)
}

fn calculate_from_valid(book: Vec<&Vec<u32>>) -> u32 {
    book.iter()
        .map(|pages| {
            let middle = pages[(pages.len() - 1) / 2];
            middle
        })
        .sum()
}
