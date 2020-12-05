use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut x = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        x %= line.len();
        if &line[x..x+1] == "#" {
            count += 1;
        }
        x += 3;
    }
    println!("{}", count);

    Ok(())
}
