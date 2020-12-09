use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    // Read file
    let range = 25;
    let mut buffer = VecDeque::new();
    let mut invalid = None;
    for line in reader.lines().map(|line| line.unwrap()) {
        let num: u64 = line.parse().unwrap();

        // Check that the next number is a sum of a recent pair of numbers
        if buffer.len() == range {
            let found = buffer.iter().enumerate()
                .any(|(i, a)| buffer.iter().skip(i+1).any(|b| a+b == num));

            if !found {
                invalid = Some(num);
                break;
            }
        }

        // Keep the last few numbers
        buffer.push_front(num);
        while buffer.len() > range {
            buffer.pop_back();
        }
    }

    println!("Part 1: {}", invalid.unwrap());

    Ok(())
}
