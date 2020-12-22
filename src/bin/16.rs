use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
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
    let mine = mine.unwrap();

    // Compare each field of each ticket to the valid ranges
    let mut errors = 0;
    let mut options = HashMap::new();  // index -> set of options
    for ticket in tickets {
        for (i, n) in ticket.iter().enumerate() {
            // Find which fields this could be
            let opts: HashSet<&String> = field_bounds.iter()
                .filter(|(_, bounds)| bounds.iter().any(|bound| bound.contains(n)))
                .map(|(field, _)| field)
                .collect();
            // If none, ignore entirely (and count for part 1)
            if opts.len() == 0 {
                errors += n;
                continue;
            }
            // Otherwise, update the overall options for this slot
            let so_far = options.entry(i).or_insert_with(|| opts.clone());
            *so_far = so_far.intersection(&opts).map(|r| *r).collect();
        }
    }

    // Deduce which field is which
    let mut mapping = HashMap::new();  // name -> index
    while options.len() > 0 {
        for (i, opts) in options.iter_mut() {
            // Remove options that have since been assigned
            opts.retain(|field| !mapping.contains_key(field));
            // Update known mappings if possible
            if opts.len() == 1 {
                mapping.insert(opts.drain().next().unwrap(), *i);
            }
        }
        // Remove used options
        options.retain(|_, opts| opts.len() > 0);
    }

    // Get the actual answer (product of all "departure" fields)
    let ans: usize = mapping.iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, i)| mine.get(*i).unwrap())
        .product();

    println!("Part 1: {}", errors);
    println!("Part 2: {}", ans);

    Ok(())
}
