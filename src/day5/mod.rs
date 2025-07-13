use std::{borrow::Cow, error::Error, io::BufRead};

use advent_of_code_2024::read_from_file;

type Rules = Vec<(u32, u32)>;
type Pages = Vec<Vec<u32>>;

pub fn exercise() {
    let res = || -> Result<u32, Box<dyn Error>> {
        let buffer = read_from_file("src/day5/input.txt")?;
        let input = get_input(buffer)?;
        let n = sum_of_rearranged_median(&input);
        Ok(n)
    };
    match res() {
        Ok(n) => println!("Success. {n}"),
        Err(e) => eprintln!("Error. {e}"),
    }
}

fn get_input<R: BufRead>(buffer: R) -> Result<(Rules, Pages), Box<dyn Error>> {
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

fn sum_of_rearranged_median(input: &(Rules, Pages)) -> u32 {
    let (rules, pages) = input;
    let mut valid_pages = Vec::new();

    for vec in pages {
        if let Some(reordered) = check_valid_page(&vec, rules) {
            valid_pages.push(reordered);
        }
    }
    calc_sum_median(&valid_pages)
}

fn check_valid_page(page: &Vec<u32>, rules: &Rules) -> Option<Vec<u32>> {
    let mut cow_page = Cow::from(page);

    loop {
        let mut is_valid = true;
        for rule in rules {
            if let (Some(first), Some(second)) = (
                cow_page.iter().position(|v| v == &rule.0),
                cow_page.iter().position(|v| v == &rule.1),
            ) {
                if first > second {
                    cow_page.to_mut().swap(first, second);
                    is_valid = false;
                }
            }
        }
        if is_valid {
            break;
        }
    }
    match cow_page {
        Cow::Borrowed(_) => None,
        Cow::Owned(vec) => Some(vec),
    }
}

fn calc_sum_median(book: &Vec<Vec<u32>>) -> u32 {
    book.iter()
        .map(|pages| {
            let middle = pages[(pages.len() - 1) / 2];
            middle
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    ///
    /// Test to verify that the input which was given by the problem will match a known output.
    #[test]
    fn test_day5_part1() {
        let res = || -> Result<u32, Box<dyn Error>> {
            let str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
            let buffer = Cursor::new(str);
            let input = get_input(buffer)?;
            let n = sum_of_rearranged_median(&input);
            Ok(n)
        };
        assert!(
            res().is_ok_and(|x| x == 123),
            "Output does not match the known answer."
        );
    }
}
