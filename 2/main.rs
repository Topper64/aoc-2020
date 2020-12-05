use std::io::{self, BufRead, BufReader};
use std::fs::File;

struct Rule {
    nums: (i32, i32),
    char: String,
}

impl Rule {
    fn new(rule: &str) -> Rule {
        // Split the rule into the counts and the character
        let mut parts = rule.split_whitespace();
        let range = parts.next().unwrap();
        let char = parts.next().unwrap().to_string();

        // Split the range into actual numbers
        let mut parts = range.split("-");
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();

        Rule {nums: (a, b), char}
    }
}

fn check(line: &str) -> bool {
    // First split into the rule and the password
    let mut parts = line.split(": ");
    let rule = Rule::new(parts.next().unwrap());
    let password = parts.next().unwrap();

    let count = password.matches(&rule.char).count() as i32;
    rule.nums.0 <= count && count <= rule.nums.1
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        if check(&line?) {
            count += 1;
        }
    }
    println!("{}", count);

    Ok(())
}
