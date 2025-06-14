mod day1;

fn main() {
    match day1::exercise() {
        Ok((distance, similarity)) => {
            println!("Distance is {:?}", distance);
            println!("Similarity is {:?}", similarity);
        }
        Err(e) => println!("Failed with error {e}"),
    }
}
