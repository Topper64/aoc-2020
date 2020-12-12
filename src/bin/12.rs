use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/12.txt")?);

    // Read file
    let mut x = 0;
    let mut y = 0;
    let mut angle = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut action = line.chars().next().unwrap();
        let value: i32 = line[1..].parse().unwrap();
        if action == 'F' {
            action = match angle {
                0 => 'E',
                90 => 'S',
                180 => 'W',
                270 => 'N',
                _ => panic!("Tried to move in direction {}", angle),
            }
        }
        match action {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'L' => angle = (angle - value).rem_euclid(360),
            'R' => angle = (angle + value).rem_euclid(360),
            _ => panic!("Unrecognised action {}", action)
        }
    }

    println!("Part 1: {}", x.abs() + y.abs());

    Ok(())
}
