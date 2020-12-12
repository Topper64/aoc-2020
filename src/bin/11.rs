use std::io::{self, BufRead, BufReader};
use std::fs::File;

type Grid = Vec<Vec<Option<bool>>>;
type UpdateFn = dyn Fn(&Grid, usize, usize) -> Option<bool>;

fn _print_grid(grid: &Grid) {
    for row in grid.iter() {
        println!("{}", row.iter().map(|seat| match seat {
            Some(true) => '#',
            Some(false) => 'L',
            None => '.',
        }).collect::<String>());
    }
}

fn count_neighbours(grid: &Grid, x: usize, y: usize) -> i32 {
    // Count number of adjacent occupied seats
    let mut count = 0;
    let height = grid.len();
    let width = grid[0].len();
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
    count
}

fn count_line_of_sight(grid: &Grid, x: usize, y: usize) -> i32 {
    // Count number of visible occupied seats
    let mut count = 0;
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let offsets = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    for (dx, dy) in &offsets {
        let mut x2 = x as i32 + dx;
        let mut y2 = y as i32 + dy;
        while 0 <= x2 && x2 < width && 0 <= y2 && y2 < height {
            if let Some(occupied) = grid[y2 as usize][x2 as usize] {
                if occupied {
                    count += 1;
                }
                break;
            }
            x2 += dx;
            y2 += dy;
        }
    }
    count
}

fn populate_grid(mut grid: Grid, update: &UpdateFn) -> Grid {
    // Apply the seating rules
    let mut changes = grid.clone();
    loop {
        // Determine which seats should change
        for (y, row) in grid.iter().enumerate() {
            for x in 0..row.len() {
                changes[y][x] = update(&grid, x, y);
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

    grid
}

fn count_occupied(grid: &Grid) -> usize {
    // Count the occupied seats
    grid.iter().fold(0, |acc, row|
        acc + row.iter().filter(|seat| **seat == Some(true)).count()
    )
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

    let grid1 = populate_grid(grid.clone(), &|grid: &Grid, x, y|
        match (grid[y][x], count_neighbours(&grid, x, y)) {
            (None, _) => None,
            (Some(false), 0) => Some(true),
            (Some(true), 4..=8) => Some(true),
            _ => Some(false),
        }
    );

    let grid2 = populate_grid(grid.clone(), &|grid: &Grid, x, y|
        match (grid[y][x], count_line_of_sight(&grid, x, y)) {
            (None, _) => None,
            (Some(false), 0) => Some(true),
            (Some(true), 5..=8) => Some(true),
            _ => Some(false),
        }
    );

    println!("Part 1: {}", count_occupied(&grid1));
    println!("Part 2: {}", count_occupied(&grid2));

    Ok(())
}
