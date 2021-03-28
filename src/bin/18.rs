use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn evaluate(expr: &str) -> i64 {
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    for mut term in expr.split_whitespace() {
        // Check for brackets
        let mut open = 0;
        while let Some(trimmed) = term.strip_prefix('(') {
            term = trimmed;
            open += 1;
        }
        let mut close = 0;
        while let Some(trimmed) = term.strip_suffix(')') {
            term = trimmed;
            close += 1;
        }

        if let Ok(mut n) = i64::from_str_radix(term, 10) {
            if open > 0 {
                // Add dummy operators to keep track of bracket depth
                while open > 1 {
                    open -= 1;
                    ops.push("(");
                }
            } else if ops.len() > 0 {
                // Eagerly apply any operator as long as no brackets are being opened
                close += 1;
            }

            while close > 0 {
                close -= 1;
                if let Some(op) = ops.pop() {
                    n = match op {
                        "+" => nums.pop().unwrap() + n,
                        "*" => nums.pop().unwrap() * n,
                        _ => n,
                    }
                }
            }

            nums.push(n);
        } else {
            ops.push(term);
        }
        // println!("{}: {} {:?}", term, ops.join(" "), nums);
    }

    nums.pop().unwrap()
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/18.txt")?);

    // Read file
    let mut tot = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        let ans = evaluate(&line);
        tot += ans;
        // println!("{} = {}", line, ans);
    }

    println!("Part 1: {}", tot);

    Ok(())
}
