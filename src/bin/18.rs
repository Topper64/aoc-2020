use std::io::{self, BufRead, BufReader};
use std::fs::File;

enum Op {
    Add,
    Mul,
    Open,
    Close,
}

enum Token {
    Num(i64),
    Op(Op),
}

fn tokenise(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for mut term in expr.split_whitespace() {
        // Check for brackets
        // Open brackets can be pushed immediately, close brackets must be
        // remembered for later
        while let Some(trimmed) = term.strip_prefix('(') {
            term = trimmed;
            tokens.push(Token::Op(Op::Open));
        }
        let mut close = 0;
        while let Some(trimmed) = term.strip_suffix(')') {
            term = trimmed;
            close += 1;
        }

        // Identify operators or numbers
        tokens.push(match term {
            "+" => Token::Op(Op::Add),
            "*" => Token::Op(Op::Mul),
            _ => Token::Num(i64::from_str_radix(term, 10).unwrap()),
        });

        // Add the closing brackets
        while close > 0 {
            close -= 1;
            tokens.push(Token::Op(Op::Close));
        }
    }
    tokens
}

fn evaluate<F, T>(expr: &str, prec: F) -> i64
where
    F: Fn(&Op) -> T,
    T: PartialOrd,
{
    let mut nums = Vec::new();
    let mut ops = Vec::new();

    // Function to apply an operator to the top two numbers
    let eval = |nums: &mut Vec<i64>, op: &Op| {
        let n1 = nums.pop().unwrap();
        let n2 = nums.pop().unwrap();
        nums.push(match op {
            Op::Add => n1 + n2,
            Op::Mul => n1 * n2,
            _ => panic!(),
        });
    };

    let tokens = tokenise(expr);
    for token in tokens.iter() {
        match token {
            Token::Num(n) => nums.push(*n),
            Token::Op(Op::Open) => ops.push(&Op::Open),
            Token::Op(Op::Close) => {
                // Evaluate all operators back to the matching open bracket
                while let Some(op) = ops.pop() {
                    if let Op::Open = op {
                        break;
                    }
                    eval(&mut nums, op);
                }
            },
            Token::Op(op) => {
                // Evaluate operators with higher or equal precedence
                let p = prec(op);
                while let Some(prev) = ops.pop() {
                    if prec(prev) >= p {
                        eval(&mut nums, prev);
                    } else {
                        ops.push(prev);
                        break;
                    }
                }
                // Remember current operator
                ops.push(op);
            },
        }
    }

    // Evaluate any remaining operators
    while let Some(op) = ops.pop() {
        eval(&mut nums, op);
    }

    nums.pop().unwrap()
}

// No precedence: a always precedes (or is equal to) b
fn no_prec(a: &Op) -> i8 {
    match a {
        Op::Open => -1,  // Must be lowest
        Op::Close => 1,  // Irrelevant, might as well be highest
        _ => 0,
    }
}

// Addition preceeds multiplication
fn add_mul(a: &Op) -> i8 {
    match a {
        Op::Open => -1,
        Op::Mul => 1,
        Op::Add => 2,
        Op::Close => 10,
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/18.txt")?);

    // Read file
    let mut part1 = 0;
    let mut part2 = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        part1 += evaluate(&line, no_prec);
        part2 += evaluate(&line, add_mul);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
