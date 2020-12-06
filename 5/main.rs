use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let highest = reader.lines()
        .map(|line| line.unwrap().chars().fold(0, |id, char|
            (id << 1) + match char {
                'B' => 1,
                'R' => 1,
                _ => 0,
            }
        ))
        .max().unwrap();
    println!("Part 1: {}", highest);

    Ok(())
}
