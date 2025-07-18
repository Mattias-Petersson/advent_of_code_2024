use std::{error::Error, io::Read};

use advent_of_code_2024::read_from_file;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Block {
    File(u32),
    Free,
}
pub fn exercise() {
    let res = || -> Result<usize, Box<dyn Error>> {
        let input = get_input()?;
        let mut mem_addresses = create_memory(&input);
        defrag_memory(&mut mem_addresses);

        let sum = mem_addresses
            .iter()
            .enumerate()
            .filter_map(|(i, block)| match block {
                Block::File(id) => Some(i * (*id as usize)),
                Block::Free => None,
            })
            .sum();
        Ok(sum)
    };

    match res() {
        Ok(n) => println!("Checksum is {n}"),
        Err(e) => eprintln!("Error. {e}"),
    }
}

fn get_input() -> Result<Vec<u32>, Box<dyn Error>> {
    let mut buffer = read_from_file("src/week2/day9/input.txt")?;
    let mut line = String::new();

    buffer.read_to_string(&mut line)?;
    let res: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    Ok(res)
}

fn create_memory(input: &Vec<u32>) -> Vec<Block> {
    let mut res = Vec::new();
    let mut id = 0;
    for (i, &n) in input.iter().enumerate() {
        let mut memory = match i % 2 {
            0 => {
                let vec = vec![Block::File(id); n as usize];
                id += 1;
                vec
            }
            1 => vec![Block::Free; n as usize],
            _ => unreachable!("The values in GF(2) are {{0, 1}}, and nothing else."),
        };
        res.append(&mut memory);
    }
    res
}

fn defrag_memory(input: &mut Vec<Block>) {
    let mut start = 0;
    let mut end = input.len() - 1;
    while start < end {
        if input[end] == Block::Free {
            end -= 1;
            continue;
        }
        if input[start] != Block::Free {
            start += 1;
            continue;
        }
        input[start] = input[end];
        input[end] = Block::Free;
        start += 1;
        end -= 1;
    }
    let first_free_mem = input.iter().position(|b| *b == Block::Free).unwrap();
    input.truncate(first_free_mem);
}
