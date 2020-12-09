use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    // Read file
    let mut instructions = Vec::new();
    let mut visited = Vec::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut parts = line.split_whitespace();

        let operation = String::from(parts.next().unwrap());
        let argument: i32 = parts.next().unwrap().parse().unwrap();

        instructions.push((operation, argument));
        visited.push(false);
    }

    let mut index = 0;
    let mut acc = 0;
    while !visited[index] {
        visited[index] = true;

        let (operation, argument) = &instructions[index];
        match operation.as_str() {
            "acc" => {
                acc += argument;
                index += 1;
            },
            "jmp" => {
                if argument > &0 {
                    index += *argument as usize;
                } else {
                    index -= -*argument as usize;
                }
            },
            "nop" => index += 1,
            _ => panic!("idk dude"),
        }
    }

    println!("Part 1: {}", acc);

    Ok(())
}
