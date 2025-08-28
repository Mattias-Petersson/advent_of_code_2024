use std::{fs, ops::Add};

struct Pair {
    x: i64,
    y: i64,
}

impl Pair {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add<i64> for Pair {
    type Output = Pair;

    fn add(self, rhs: i64) -> Pair {
        Pair {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn parse_pair(line: &str) -> Pair {
    let nums: Vec<i64> = line
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    Pair::new(nums[0], nums[1])
}

fn read_input() -> Vec<(Pair, Pair, Pair)> {
    let mut res = Vec::new();
    let constant_factor = 10000000000000;
    let input = fs::read_to_string("src/week2/day13/input.txt").unwrap();
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());
    while let (Some(first_line), Some(second_line), Some(prize_line)) =
        (lines.next(), lines.next(), lines.next())
    {
        let first_equation = parse_pair(first_line);
        let second_equation = parse_pair(second_line);
        let mut ans = parse_pair(prize_line);
        ans = ans + constant_factor; // For part 2
        res.push((first_equation, second_equation, ans));
    }
    res
}

pub fn exercise() {
    let mut count = 0;
    let problems = read_input();
    for i in problems {
        if let Some(solved) = cramers_rule(i) {
            count += solved.x * 3 + solved.y;
        }
    }
    println!("{}", count);
}

fn cramers_rule((first, second, res): (Pair, Pair, Pair)) -> Option<Pair> {
    let determinant = first.x * second.y - first.y * second.x;
    let numerator_x = res.x * second.y - res.y * second.x;
    let numerator_y = res.y * first.x - res.x * first.y;

    if numerator_x % determinant != 0 || numerator_y % determinant != 0 {
        // Either no solution or infinite, either way not relevant to the problem.
        return None;
    }

    Some(Pair::new(
        numerator_x / determinant,
        numerator_y / determinant,
    ))
}
