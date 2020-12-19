use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/15.txt")?);

    // Read file
    let mut nums = HashMap::new();
    let mut last = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        for (i, num) in line.split(',').enumerate() {
            if i > 0 {
                nums.insert(last, i);
            }
            last = num.parse().unwrap();
        }
    }

    // Play the game
    for i in nums.len()+1..2020 {
        let j = i - nums.get(&last).unwrap_or(&i);
        nums.insert(last, i);
        last = j;
    }

    println!("Part 1: {}", last);

    Ok(())
}
