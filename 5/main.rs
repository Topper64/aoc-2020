use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering::{Less, Greater};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    // Read file
    let mut nums: Vec<usize> = reader.lines()
        .map(|line| line.unwrap().chars().fold(0, |id, char|
            (id << 1) + match char {
                'B' => 1,
                'R' => 1,
                _ => 0,
            }
        ))
        .collect();

    // Since we're missing a single value, there is a point where the parity of a value compared
    // to its index will change: find it with a binary search
    nums.sort();
    let parity = nums[0] & 1;
    let pairs: Vec<_> = nums.iter().enumerate().collect();
    let i = pairs.binary_search_by(|(i, n)| match (i ^ *n ^ parity) & 1 {
        0 => Less,
        _ => Greater,
    }).err().unwrap();

    println!("Part 1: {}", nums.last().unwrap());
    println!("Part 2: {}", nums[i] - 1);

    Ok(())
}
