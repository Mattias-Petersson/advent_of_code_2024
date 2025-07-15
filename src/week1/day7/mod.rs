use std::{error::Error, io::BufRead};

use advent_of_code_2024::read_from_file;
struct Problem {
    result: u64,
    terms: Vec<u64>,
}
impl Problem {
    fn can_solve(&self) -> bool {
        self.try_operations(self.terms[0], 1)
    }

    fn try_operations(&self, current: u64, index: usize) -> bool {
        if index == self.terms.len() {
            return current == self.result;
        }

        let next = self.terms[index];
        if self.try_operations(current + next, index + 1) {
            return true;
        }
        if self.try_operations(current * next, index + 1) {
            return true;
        }

        false
    }
}

pub fn exercise() {
    let res = || -> Result<u64, Box<dyn Error>> {
        let input = get_input()?;
        let sum_of_valid: u64 = input
            .iter()
            .filter(|p| p.can_solve())
            .map(|p| p.result)
            .sum();
        Ok(sum_of_valid)
    };
    match res() {
        Ok(i) => println!("Success. {i}"),
        Err(e) => eprintln!("Error. {e}"),
    }
}

fn get_input() -> Result<Vec<Problem>, Box<dyn Error>> {
    let buffer = read_from_file("src/week1/day7/input.txt")?;
    buffer
        .lines()
        .map(|line| {
            let line = line?;
            let (result_str, terms_str) = line.split_once(':').unwrap();
            // Remove the first element of terms_str, as that is a ':' due to the split_once.
            let terms = terms_str[1..]
                .split_whitespace()
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Problem {
                result: result_str.parse()?,
                terms,
            })
        })
        .collect()
}
