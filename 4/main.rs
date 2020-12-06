use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn check(fields: Vec<String>) -> bool {
    let mut fieldmap = HashMap::new();
    for field in fields {
        let mut keyval = field.split(":");
        let key = String::from(keyval.next().unwrap());
        let val = String::from(keyval.next().unwrap());
        fieldmap.insert(key, val);
    }

    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let optional = [false, false, false, false, false, false, false, true];
    keys.iter().zip(optional.iter())
        .all(|(key, opt)| *opt || fieldmap.contains_key(*key))
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut lines = reader.lines().map(|line| line.unwrap()).peekable();
    while lines.peek().is_some() {
        let mut fields: Vec<String> = Vec::new();
        let entry_lines = (&mut lines).take_while(|line| line.len() > 0);
        for line in entry_lines {
            fields.extend(line.split_whitespace().map(String::from));
        }
        if check(fields) {
            count += 1;
        }
    }
    println!("{}", count);

    Ok(())
}
