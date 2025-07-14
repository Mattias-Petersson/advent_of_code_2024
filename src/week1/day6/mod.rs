use std::{collections::HashSet, error::Error};

use advent_of_code_2024::get_input_to_char_array;

type Input = Vec<Vec<char>>;
type Position = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }
}
pub fn exercise() {
    let res = || -> Result<usize, Box<dyn Error>> {
        let input: Input = get_input_to_char_array("src/week1/day6/input.txt")?;
        let start_dir = Direction::UP;
        let start_pos = find_start_position(&input).unwrap();
        let n = move_guard(&input, start_pos, start_dir);
        Ok(n)
    };
    match res() {
        Ok(i) => println!("Success. {i}"),
        Err(e) => eprintln!("Error. {e}"),
    }
}

fn find_start_position(input: &Input) -> Option<Position> {
    for (row, line) in input.iter().enumerate() {
        if let Some(col) = line.iter().position(|&c| c == '^') {
            return Some((row, col));
        }
    }
    None
}

/// Used for part 2, iterates over all possible permutations, incredibly inefficient
/// TODO: Improve this using some graph algorithm.
#[allow(dead_code)]
fn naive_approach(input: &Input, start_pos: Position, start_dir: Direction) -> usize {
    let mut loop_count = 0;
    let mut mutable_input = input.clone();

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == '#' || (row, col) == start_pos {
                continue;
            }
            mutable_input[row][col] = '#';

            let mut pos = start_pos;
            let mut dir = start_dir;
            let mut visited = HashSet::new();
            let mut is_loop = false;

            loop {
                if visited.contains(&(pos, dir)) {
                    is_loop = true;
                    break;
                }
                visited.insert((pos, dir));

                if let Some((new_pos, new_dir)) = can_move(&mutable_input, pos, dir) {
                    pos = new_pos;
                    dir = new_dir;
                } else {
                    break;
                }
            }

            if is_loop {
                loop_count += 1;
            }
            mutable_input[row][col] = '.';
        }
    }
    loop_count
}

fn move_guard(input: &Input, pos: Position, dir: Direction) -> usize {
    let mut distinct: HashSet<Position> = HashSet::new();
    let mut pos = pos;
    distinct.insert(pos);
    let mut dir = dir;
    loop {
        if let Some((new_pos, new_dir)) = can_move(input, pos, dir) {
            pos = new_pos;
            dir = new_dir;
            distinct.insert(pos);
        } else {
            break;
        }
    }
    distinct.len()
}

fn can_move(input: &Input, pos: Position, dir: Direction) -> Option<(Position, Direction)> {
    let (row, col) = pos;
    let mut current_dir = dir;

    for _ in 0..3 {
        let (row_delta, col_delta) = current_dir.delta();
        let new_row = row.checked_add_signed(row_delta)?;
        let new_col = col.checked_add_signed(col_delta)?;

        if new_row >= input.len() || new_col >= input[0].len() {
            return None;
        }

        if input[new_row][new_col] != '#' {
            return Some(((new_row, new_col), current_dir));
        }

        current_dir = current_dir.turn_right();
    }
    None
}
