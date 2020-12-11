use std::io::{self, BufRead, BufReader};
use std::fs::File;

type Grid = Vec<Vec<Option<bool>>>;

fn print_grid(grid: &Grid) {
    for row in grid.iter() {
        println!("{}", row.iter().map(|seat| match seat {
            Some(true) => '#',
            Some(false) => 'L',
            None => '.',
        }).collect::<String>());
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/11.txt")?);

    // Read file
    let mut grid = Vec::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(match char {
                'L' => Some(false),
                _ => None,
            });
        }
        grid.push(row);
    }

    // Apply the seating rules
    let height = grid.len();
    let width = grid[0].len();
    let mut changes = grid.clone();
    loop {
        for (y, row) in grid.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                if seat.is_none() {
                    continue;
                }

                // Count number of adjacent occupied seats
                let mut count = 0;
                let left = if x > 0 {x - 1} else {0};
                let bottom = if y > 0 {y - 1} else {0};
                let right = if x < width-1 {x + 2} else {width};
                let top = if y < height-1 {y + 2} else {height};
                for x2 in left..right {
                    for y2 in bottom..top {
                        if (x2 != x || y2 != y) && grid[y2][x2] == Some(true) {
                            count += 1;
                        }
                    }
                }

                // Determine whether this cell should change
                let seat = seat.unwrap();
                changes[y][x] = match (seat, count) {
                    (false, 0) => Some(true),
                    (true, 4..=8) => Some(true),
                    _ => Some(false),
                }
            }
        }

        // Apply the changes
        let mut changed = false;
        for (row, chrow) in grid.iter_mut().zip(changes.iter()) {
            for (seat, change) in row.iter_mut().zip(chrow.iter()) {
                if change.is_some() {
                    changed |= change.unwrap();
                    *seat = Some(seat.unwrap() ^ change.unwrap());
                }
            }
        }

        if !changed {
            break;
        }
    }

    // Count the occupied seats
    let mut occupied = 0;
    for row in grid.iter() {
        for cell in row.iter() {
            if cell == &Some(true) {
                occupied += 1;
            }
        }
    }

    print_grid(&grid);
    println!("Part 1: {}", occupied);

    Ok(())
}
