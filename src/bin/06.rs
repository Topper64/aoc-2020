use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/06.txt")?);

    let mut lines = reader.lines().map(|line| line.unwrap()).peekable();
    let mut count1 = 0;
    let mut count2 = 0;
    while lines.peek().is_some() {
        let mut answers: HashMap<_, _> = HashMap::new();
        let mut npeople = 0;
        for line in (&mut lines).take_while(|line| line.len() > 0) {
            npeople += 1;
            for char in line.chars() {
                *answers.entry(char).or_insert(0) += 1;
            }
        }
        count1 += answers.len();
        count2 += answers.values().filter(|ans| *ans == &npeople).count();
    }

    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);

    Ok(())
}
