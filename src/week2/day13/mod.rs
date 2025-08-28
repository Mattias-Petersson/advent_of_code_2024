use std::fs;

struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
fn parse_pair(line: &str) -> Pair {
    let nums: Vec<i32> = line
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    Pair::new(nums[0], nums[1])
}

fn read_input() -> Vec<(Pair, Pair, Pair)> {
    let mut res = Vec::new();
    let input = fs::read_to_string("src/week2/day13/input.txt").unwrap();
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());
    while let (Some(first_line), Some(second_line), Some(prize_line)) =
        (lines.next(), lines.next(), lines.next())
    {
        let first_equation = parse_pair(first_line);
        let second_equation = parse_pair(second_line);
        let ans = parse_pair(prize_line);
        res.push((first_equation, second_equation, ans));
    }
    res
}

pub fn exercise() {
    let mut count = 0;
    let problems = read_input();
    // println!("{}", x1);
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
        // Either no solution or infinite, not relevant to the problem.
        return None;
    }

    Some(Pair::new(
        numerator_x / determinant,
        numerator_y / determinant,
    ))
}
