use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("inputs/01.txt")?;
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();
    let mut pairs = Vec::new();
    for line in reader.lines() {
        // Parse as a number
        let n = line?.parse::<i32>().unwrap();

        // Check all possible sums of 3 numbers
        for (sum, prod) in &pairs {
            if sum + n == 2020 {
                println!("Part 2: {}", prod * n);
            }
        }

        // Update rolling sums and products
        for m in &numbers {
            let sum = m + n;
            let prod = m * n;
            if sum == 2020 {
                println!("Part 1: {}", prod);
            }
            pairs.push((sum, prod));
        }

        // Remember for future loops
        numbers.push(n);
    }

    Ok(())
}
