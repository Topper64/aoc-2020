use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn in_range(val: Option<i32>, low: i32, high: i32) -> bool {
    match val {
        Some(val) => low <= val && val <= high,
        _ => false,
    }
}

fn is_hexcode(val: &str) -> bool {
    if val.len () != 7 {
        return false
    }

    // Check first character is #
    let mut chars = val.chars();
    if chars.next() != Some('#') {
        return false
    }

    // Check remaining characters are 0-9 a-f
    if !chars.all(|char|
        char.is_ascii_hexdigit()
        && (char.is_ascii_digit() || char.is_lowercase())
    ) {
        return false
    }

    true
}

fn check_field(key: &str, val: &str) -> bool {
    match key {
        // Birth year: 1920-2002
        "byr" => in_range(val.parse().ok(), 1920, 2002),
        // Issue year: 2010-2020
        "iyr" => in_range(val.parse().ok(), 2010, 2020),
        // Expiration year: 2020-2030
        "eyr" => in_range(val.parse().ok(), 2020, 2030),

        // Height: 150cm-193cm or 59in-76in
        "hgt" => {
            if let Some(digits) = val.strip_suffix("cm") {
                in_range(digits.parse().ok(), 150, 193)
            } else if let Some(digits) = val.strip_suffix("in") {
                in_range(digits.parse().ok(), 59, 76)
            } else {
                false
            }
        },

        // Hair colour: 6-digit hex code
        "hcl" => is_hexcode(val),
        // Eye colour: amb blu brn gry grn hzl oth
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val),

        // Passport ID: exactly 9 digits
        "pid" => val.len() == 9 && val.chars().all(|char| char.is_ascii_digit()),
        // Country ID: ignored
        "cid" => true,

        // Anything else: error
        _ => false,
    }
}

fn check_passport(fields: &Vec<String>) -> Option<bool> {
    let mut fieldmap = HashMap::new();
    for field in fields {
        let mut keyval = field.split(":");
        let key = keyval.next().unwrap();
        let val = keyval.next().unwrap();
        fieldmap.insert(key, check_field(key, val));
    }

    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let optional = [false, false, false, false, false, false, false, true];
    let mut result = Some(true);
    for (key, opt) in keys.iter().zip(optional.iter()) {
        if *opt {
            continue;
        }
        result = match (result, fieldmap.get(key)) {
            (Some(result), Some(valid)) => Some(result & valid),
            _ => None,
        }
    }
    result
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count1 = 0;
    let mut count2 = 0;
    let mut lines = reader.lines().map(|line| line.unwrap()).peekable();
    while lines.peek().is_some() {
        let mut fields: Vec<String> = Vec::new();
        let entry_lines = (&mut lines).take_while(|line| line.len() > 0);
        for line in entry_lines {
            fields.extend(line.split_whitespace().map(String::from));
        }
        if let Some(valid) = check_passport(&fields) {
            count1 += 1;
            if valid {
                count2 += 1;
            }
        }
    }
    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);

    Ok(())
}
