use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/14.txt")?);

    // Read file
    let mut mask: u64 = 0;
    let mut default: u64 = 0;
    let mut mem = HashMap::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == "mask" {
            mask = 0;
            default = 0;
            for char in value.chars() {
                mask <<= 1;
                default <<= 1;
                match char {
                    'X' => mask += 1,
                    '1' => default += 1,
                    _ => {},
                }
            }
        } else {
            // This looks like "mem[\d+]" but we don't really need regex to pick out the number
            let key: String = key.chars().filter(|c| c.is_ascii_digit()).collect();
            let value: u64 = value.parse().unwrap();
            mem.insert(key, default + (value & mask));
        }
    }

    println!("Part 1: {}", mem.values().sum::<u64>());

    Ok(())
}
