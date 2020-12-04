use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();
    for line in reader.lines() {
        // Parse as a number
        let n = line?.parse::<i32>().unwrap();

        // Check all previous numbers
        for m in &numbers {
            if m + n == 2020 {
                println!("{}", m * n);
            }
        }

        // Remember for future loops
        numbers.push(n);
    }

    Ok(())
}
