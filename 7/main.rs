use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::{HashSet, HashMap};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    // Read file
    let mut relations = HashMap::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        let line = line.trim_matches('.');

        let mut container = None;
        let mut colour_words = Vec::new();
        let mut count = None;
        for word in line.split_whitespace().map(|word| word.trim_matches(',')) {
            match word {
                // The word "bag" or "bags" marks the end of a colour
                "bag" | "bags" => {
                    let colour = colour_words.join(" ");
                    colour_words.clear();
                    if let Some(key) = &container {
                        let contains = relations.entry(String::from(key))
                            .or_insert(HashMap::new());
                        if count > Some(0) {
                            contains.insert(colour, count.unwrap());
                        }
                    } else {
                        container = Some(colour);
                    }
                },

                // Numbers
                "no" => count = Some(0),
                _ if word.chars().all(|char| char.is_ascii_digit()) => count = word.parse().ok(),

                // This always appears so adds no information
                "contain" | "contains" => (),

                // Other words must be part of a colour - save for later
                _ => colour_words.push(word),
            };
        }
    }

    // Iteratively find all bags that can contain a "shiny gold" bag
    let mut has_gold = HashSet::new();
    has_gold.insert(String::from("shiny gold"));
    let mut size = None;
    while match size {None => true, Some(n) => has_gold.len() > n} {
        size = Some(has_gold.len());
        for (outer, inners) in relations.iter() {
            if inners.keys().any(|colour| has_gold.contains(colour)) {
                has_gold.insert(String::from(outer));
            }
        }
    }

    // Subtract 1: the shiny gold bag itself
    let size = size.unwrap() - 1;

    // Count how many total bags each bag contains
    let mut counts = HashMap::new();
    let mut done = false;
    while !done {
        done = true;

        // Identify all the ones that we've counted all children of
        let mut empty = Vec::new();
        for (inner, inners) in relations.iter() {
            if inners.is_empty() {
                empty.push(String::from(inner));
            }
        }

        for inner in empty.iter() {
            // Remove it so we don't check again next time round
            relations.remove(inner);

            // Ensure this colour has a count
            let count = *counts.entry(String::from(inner)).or_insert(0u64);

            // Update the counts of anything containing it
            for (outer, inners) in relations.iter_mut() {
                if let Some(n) = inners.remove(inner) {
                    *counts.entry(String::from(outer)).or_insert(0) += (count + 1) * n;
                    done = false;
                }
            }
        }
    }

    println!("Part 1: {}", size);
    println!("Part 2: {}", counts.get("shiny gold").unwrap());

    Ok(())
}
