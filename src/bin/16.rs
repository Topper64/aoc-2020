use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

fn main() -> io::Result<()> {
    // Read file
    let reader = BufReader::new(File::open("inputs/16.txt")?);
    let mut lines = reader.lines().map(|line| line.unwrap());

    // Get field ranges
    let patt = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut field_bounds = HashMap::new();
    for line in (&mut lines).take_while(|line| line.len() > 0) {
        let mut parts = line.split(": ");
        let key = String::from(parts.next().unwrap());
        let value = parts.next().unwrap();

        let mut bounds = Vec::new();
        for m in patt.captures_iter(value) {
            let i: usize = m.get(1).unwrap().as_str().parse().unwrap();
            let j: usize = m.get(2).unwrap().as_str().parse().unwrap();
            bounds.push(i..j+1);
        }

        field_bounds.insert(key, bounds);
    }

    // Get tickets
    let mut mine = None;
    let mut tickets = Vec::new();
    for line in lines {
        let ticket: Vec<usize> = match line.as_str() {
            // These lines aren't tickets
            "your ticket:" | "nearby tickets:" | "" => continue,
            // Anything else is: parse into actual numbers
            _ => line.split(',').map(|num| num.parse().unwrap()).collect()
        };

        if mine.is_none() {
            // First ticket is mine
            mine = Some(ticket);
        } else {
            tickets.push(ticket);
        }
    }

    // Count the errors
    let errors = tickets.iter().map(
        |ticket| ticket.iter().filter(
            |n| !field_bounds.values().any(
                |bounds| bounds.iter().any(|bound| bound.contains(n))
            )
        ).sum::<usize>()
    ).sum::<usize>();

    println!("Part 1: {}", errors);

    Ok(())
}
