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
        for word in line.split_whitespace().map(|word| word.trim_matches(',')) {
            match word {
                // The word "bag" or "bags" marks the end of a colour
                "bag" | "bags" => {
                    let colour = colour_words.join(" ");
                    colour_words.clear();
                    if let Some(key) = &container {
                        relations.entry(String::from(key))
                            .or_insert(HashSet::new())
                            .insert(colour);
                    } else {
                        container = Some(colour);
                    }
                },

                // Numbers
                "no" => (),
                _ if word.chars().all(|char| char.is_ascii_digit()) => (),

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
            if inners.iter().any(|colour| has_gold.contains(colour)) {
                has_gold.insert(String::from(outer));
            }
        }
    }

    // Subtract 1: the shiny gold bag itself
    let size = size.unwrap() - 1;

    println!("Part 1: {}", size);

    Ok(())
}
