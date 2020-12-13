use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/13.txt")?);

    // Read file
    let mut lines = reader.lines().map(|line| line.unwrap());
    let start: i32 = lines.next().unwrap().parse().unwrap();
    let mut soonest = None;
    for freq in lines.next().unwrap().split(',') {
        if let Ok(freq) = freq.parse::<i32>() {
            let wait = freq - start.rem_euclid(freq);
            soonest = match soonest {
                Some((w, _)) if w < wait => soonest,
                _ => Some((wait, freq)),
            }
        }
    }

    let (wait, freq) = soonest.unwrap();
    println!("Part 1: {}", wait * freq);

    Ok(())
}
