use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut counts = [0; 5];
    let mut xs = [0; 5];
    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let width = line.len();
        for (i, (dx, dy)) in slopes.iter().enumerate() {
            if row % dy != 0 {
                continue;
            }

            xs[i] %= width;
            if &line[xs[i]..xs[i]+1] == "#" {
                counts[i] += 1;
            }
            xs[i] += dx;
        }
    }
    println!("Part 1: {}", counts[1]);
    println!("Part 2: {}", counts.iter().product::<u64>());

    Ok(())
}
