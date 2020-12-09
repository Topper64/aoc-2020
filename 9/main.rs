use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    // Read file
    let range = 25;
    let mut numbers = VecDeque::new();
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

        // Update numbers
        buffer.push_front(num);
        while buffer.len() > range {
            numbers.push_front(buffer.pop_back().unwrap());
        }
    }

    // Now try to find contiguous numbers that sum to the invalid one
    let invalid = invalid.unwrap();
    numbers.append(&mut buffer);
    let mut sum = 0;
    while sum != invalid {
        if sum < invalid {
            let n = numbers.pop_back().unwrap();
            sum += n;
            buffer.push_front(n);
        } else if sum > invalid {
            let n = buffer.pop_back().unwrap();
            sum -= n;
        }
    }
    let key = buffer.iter().min().unwrap() + buffer.iter().max().unwrap();

    println!("Part 1: {}", invalid);
    println!("Part 2: {}", key);

    Ok(())
}
