use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/14.txt")?);

    // Read file
    let mut mask: u64 = 0;
    let mut default: u64 = 0;
    let mut options = Vec::new();
    let mut mem1 = HashMap::new();
    let mut mem2 = HashMap::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        if key == "mask" {
            mask = 0;
            default = 0;
            options.clear();
            options.push(0u64);

            for char in value.chars() {
                mask <<= 1;
                default <<= 1;
                for option in options.iter_mut() {
                    *option <<= 1;
                }

                if char == 'X' {
                    mask += 1;
                    let newopts: Vec<_> = options.iter().map(|opt| opt + 1).collect();
                    options.extend(newopts);
                } else if char == '1' {
                    default += 1;
                }
            }
        } else {
            // This looks like "mem[\d+]" but we don't really need regex to pick out the number
            let key: String = key.chars().filter(|c| c.is_ascii_digit()).collect();
            let key: u64 = key.parse().unwrap();
            let value: u64 = value.parse().unwrap();
            mem1.insert(key, default | (value & mask));
            for option in &options {
                mem2.insert(default | option | (key & !mask), value);
            }
        }
    }

    println!("Part 1: {}", mem1.values().sum::<u64>());
    println!("Part 2: {}", mem2.values().sum::<u64>());

    Ok(())
}
