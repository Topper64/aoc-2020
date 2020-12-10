use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/10.txt")?);

    // Read file
    let mut nums: Vec<_> = reader.lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Insert 0 (the outlet), and a number 3 higher than the maximum (the device)
    nums.push(0);
    nums.push(nums.iter().max().unwrap() + 3);

    // Count the differences of 1 and 3
    nums.sort();
    let diffs: Vec<_> = nums.iter().zip(nums.iter().skip(1)).map(|(a, b)| b - a).collect();
    let ones = diffs.iter().filter(|d| **d == 1).count();
    let threes = diffs.iter().filter(|d| **d == 3).count();

    // Count the ways of making a valid chain
    let mut ways: Vec<u64> = vec![0; nums.len()];
    ways[0] = 1;
    for i in 0..ways.len() {
        let a = nums[i];
        let reachable = nums.iter().skip(i+1).take_while(|b| *b - a <= 3).count();
        for j in i+1..i+1+reachable {
            ways[j] += ways[i];
        }
    }

    println!("Part 1: {}", ones * threes);
    println!("Part 2: {}", ways.last().unwrap());

    Ok(())
}
