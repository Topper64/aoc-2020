use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::ops;


type Instruction = (char, i32);
type Instructions = Vec<Instruction>;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn abs(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate(self, angle: i32) -> Point {
        match angle.rem_euclid(360) {
            0 => Point {x: self.x, y: self.y},
            90 => Point {x: self.y, y: -self.x},
            180 => Point {x: -self.x, y: -self.y},
            270 => Point {x: -self.y, y: self.x},
            _ => panic!("Tried to move in direction {}", angle),
        }
    }
}

impl ops::Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Mul<i32> for &Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {x: self.x * other, y: self.y * other}
    }
}


fn follow_rules_1(instructions: &Instructions) -> Point {
    let mut pos = Point {x: 0, y: 0};
    let mut angle = 0;

    for (action, value) in instructions {
        // Rewrite F (forward) in terms of N S E W
        let action = match *action {
            'F' => match angle {
                0 => 'E',
                90 => 'S',
                180 => 'W',
                270 => 'N',
                _ => panic!("Tried to move in direction {}", angle),
            },
            a => a,
        };

        match action {
            'N' => pos.y += value,
            'S' => pos.y -= value,
            'E' => pos.x += value,
            'W' => pos.x -= value,
            'L' => angle = (angle - value).rem_euclid(360),
            'R' => angle = (angle + value).rem_euclid(360),
            _ => panic!("Unrecognised action {}", action),
        }
    }

    pos
}

fn follow_rules_2(instructions: &Instructions) -> Point {
    let mut pos = Point {x: 0, y: 0};
    let mut dir = Point {x: 10, y: 1};

    for (action, value) in instructions {
        match action {
            'N' => dir.y += value,
            'S' => dir.y -= value,
            'E' => dir.x += value,
            'W' => dir.x -= value,
            'L' => dir = dir.rotate(-*value),
            'R' => dir = dir.rotate(*value),
            'F' => pos = &pos + &(&dir * *value),
            _ => panic!("Unrecognised action {}", action),
        }
        // println!("{} {} : ({}, {}) ({}, {})", action, value, pos.x, pos.y, dir.x, dir.y);
    }

    pos
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/12.txt")?);

    // Read file
    let mut instructions = Vec::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        let action = line.chars().next().unwrap();
        let value: i32 = line[1..].parse().unwrap();
        instructions.push((action, value));
    }

    let pos = follow_rules_1(&instructions);
    println!("Part 1: {}", pos.abs());

    let pos = follow_rules_2(&instructions);
    println!("Part 2: {}", pos.abs());

    Ok(())
}
