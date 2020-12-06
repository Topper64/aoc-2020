use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let mut lines = reader.lines().map(|line| line.unwrap()).peekable();
    let mut count = 0;
    while lines.peek().is_some() {
        let mut answers: HashSet<char> = HashSet::new();
        for line in (&mut lines).take_while(|line| line.len() > 0) {
            answers.extend(line.chars());
        }
        count += answers.len();
    }

    println!("Part 1: {}", count);

    Ok(())
}
