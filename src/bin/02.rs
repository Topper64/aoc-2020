use std::io::{self, BufRead, BufReader};
use std::fs::File;

struct Rule {
    nums: (usize, usize),
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

fn check1(rule: &Rule, password: &str) -> bool {
    let count = password.matches(&rule.char).count();
    rule.nums.0 <= count && count <= rule.nums.1
}

fn check2(rule: &Rule, password: &str) -> bool {
    let mut result = false;
    for i in &[rule.nums.0, rule.nums.1] {
        let char = &password[*i - 1 .. *i];
        result ^= char == rule.char;
    }
    result
}

fn main() -> io::Result<()> {
    let file = File::open("inputs/02.txt")?;
    let reader = BufReader::new(file);

    let mut count1 = 0;
    let mut count2 = 0;
    for line in reader.lines() {
        // First split into the rule and the password
        let line = line.unwrap();
        let mut parts = line.split(": ");
        let rule = Rule::new(parts.next().unwrap());
        let password = parts.next().unwrap();

        if check1(&rule, &password) {
            count1 += 1;
        }
        if check2(&rule, &password) {
            count2 += 1;
        }
    }
    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);

    Ok(())
}
