use std::io::{self, BufRead, BufReader};
use std::fs::File;

enum Token {
    Num(i64),
    Add,
    Mul,
    Open,
    Close,
}

fn tokenise(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for mut term in expr.split_whitespace() {
        // Check for brackets
        // Open brackets can be pushed immediately, close brackets must be
        // remembered for later
        while let Some(trimmed) = term.strip_prefix('(') {
            term = trimmed;
            tokens.push(Token::Open);
        }
        let mut close = 0;
        while let Some(trimmed) = term.strip_suffix(')') {
            term = trimmed;
            close += 1;
        }

        // Identify operators or numbers
        tokens.push(match term {
            "+" => Token::Add,
            "*" => Token::Mul,
            _ => Token::Num(i64::from_str_radix(term, 10).unwrap()),
        });

        // Add the closing brackets
        while close > 0 {
            close -= 1;
            tokens.push(Token::Close);
        }
    }
    tokens
}

fn evaluate(expr: &str) -> i64 {
    let mut nums = Vec::new();
    let mut ops = Vec::new();

    // Function to apply an operator to the top two numbers
    let eval = |nums: &mut Vec<i64>, op: &Token| {
        let n1 = nums.pop().unwrap();
        let n2 = nums.pop().unwrap();
        nums.push(match op {
            Token::Add => n1 + n2,
            Token::Mul => n1 * n2,
            _ => panic!(),
        });
    };

    let tokens = tokenise(expr);
    for token in tokens.iter() {
        match token {
            Token::Num(n) => nums.push(*n),
            Token::Open => ops.push(token),
            Token::Close => {
                // Evaluate all operators back to the matching open bracket
                while let Some(op) = ops.pop() {
                    if let Token::Open = op {
                        break;
                    }
                    eval(&mut nums, op);
                }
            },
            _ => {
                // Evaluate operators with higher or equal precedence
                while let Some(op) = ops.pop() {
                    if let Token::Open = op {
                        ops.push(op);
                        break;
                    }
                    eval(&mut nums, op);
                }
                // Remember current operator
                ops.push(token);
            },
        }
    }

    // Evaluate any remaining operators
    while let Some(op) = ops.pop() {
        eval(&mut nums, op);
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
