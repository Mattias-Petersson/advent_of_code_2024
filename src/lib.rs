use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn read_from_file(path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let input: File =
        File::open(path).map_err(|err| format!("Failed to open {}: {}", path, err))?;
    let buffer: BufReader<File> = BufReader::new(input);
    Ok(buffer)
}
