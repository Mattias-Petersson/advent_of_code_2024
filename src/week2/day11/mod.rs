use std::{collections::HashMap, error::Error, fs};

type MemoMap = HashMap<(u64, u32), u64>;

pub fn exercise() {
    let res = || -> Result<u64, Box<dyn Error>> {
        let input = get_input()?;
        let mut memo_map = HashMap::new();

        let total = input
            .into_iter()
            .map(|n| expand(n, 75, &mut memo_map))
            .sum();

        Ok(total)
    };

    match res() {
        Ok(n) => println!("{}", n),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_input() -> Result<Vec<u64>, Box<dyn Error>> {
    let content = fs::read_to_string("src/week2/day11/input.txt")?;
    let nums = content
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(nums)
}

fn expand(n: u64, steps: u32, memo: &mut MemoMap) -> u64 {
    if steps == 0 {
        return 1;
    }

    if let Some(&cached) = memo.get(&(n, steps)) {
        return cached;
    }

    let result = match n {
        0 => expand(1, steps - 1, memo),
        n if is_even_digits(n) => {
            let (left, right) = split_number(n);
            expand(left, steps - 1, memo) + expand(right, steps - 1, memo)
        }
        _ => expand(n * 2024, steps - 1, memo),
    };

    memo.insert((n, steps), result);
    result
}

fn is_even_digits(n: u64) -> bool {
    n.to_string().len() % 2 == 0
}

fn split_number(n: u64) -> (u64, u64) {
    let s = n.to_string();
    let mid = s.len() / 2;
    let left = s[..mid].parse().unwrap();
    let right = s[mid..].parse().unwrap();
    (left, right)
}
