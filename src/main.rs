mod day1;

fn main() {
    match day1::part1() {
        Ok(distance) => {
            println!("Distance is {:?}", distance);
        }
        Err(e) => println!("Failed with error {e}"),
    }
}
