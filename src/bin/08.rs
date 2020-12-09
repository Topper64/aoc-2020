use std::io::{self, BufRead, BufReader};
use std::fs::File;

type Int = i32;
type Instructions = Vec<(String, Int)>;

fn check(instructions: &Instructions, corrupted: Option<usize>) -> Result<Int, Int> {
    let mut visited = vec![false; instructions.len()];

    let len = instructions.len();
    let mut index = 0;
    let mut acc = 0;
    while index < len && !visited[index] {
        visited[index] = true;

        let (operation, argument) = &instructions[index];
        let operation = operation.as_str();

        // Apply corruption
        let operation = match corrupted == Some(index) {
            true => match operation {
                "jmp" => "nop",
                "nop" => "jmp",
                _ => operation,
            }
            _ => operation,
        };

        // Carry out instruction
        match operation {
            "acc" => {
                acc += argument;
                index += 1;
            },

            "jmp" => {
                let offset = *argument;
                if offset > 0 {
                    index += offset as usize;
                } else {
                    let offset = -offset as usize;
                    if index < offset {
                        break;
                    }
                    index -= offset;
                }
            },

            "nop" => index += 1,

            _ => panic!("idk dude"),
        }
    }

    if index == len {
        Ok(acc)
    } else {
        Err(acc)
    }
}

fn try_fix(instructions: &Instructions) -> Option<Int> {
    for i in 0..instructions.len() {
        match check(&instructions, Some(i)) {
            Ok(acc) => return Some(acc),
            _ => (),
        };
    }
    None
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/08.txt")?);

    // Read file
    let mut instructions = Vec::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut parts = line.split_whitespace();

        let operation = String::from(parts.next().unwrap());
        let argument: Int = parts.next().unwrap().parse().unwrap();

        instructions.push((operation, argument));
    }

    println!("Part 1: {}", check(&instructions, None).unwrap_err());
    println!("Part 2: {}", try_fix(&instructions).unwrap());

    Ok(())
}
