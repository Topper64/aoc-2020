use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::fmt;
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Default)]
struct Grid3d<T> {
    cells: HashMap<(i32, i32, i32), T>,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
}

impl From<File> for Grid3d<bool> {
    fn from(file: File) -> Grid3d<bool> {
        let reader = BufReader::new(file);

        let mut cells = HashMap::new();
        let mut xmax = 0;
        let mut ymax = 0;
        for (y, line) in reader.lines().map(|line| line.unwrap()).enumerate() {
            let y = y as i32;
            for (x, char) in line.char_indices() {
                let x = x as i32;
                let i = (x, y, 0);
                cells.insert(i, char == '#');
                if x > xmax {
                    xmax = x;
                }
            }
            ymax = y;
        }

        Grid3d {
            cells,
            xmax,
            ymax,
            ..Default::default()
        }
    }
}

impl fmt::Display for Grid3d<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &(self.zmin..=self.zmax)
                .map(|z| format!(
                    "z={}\n{}",
                    z,
                    (self.ymin..=self.ymax)
                        .map(|y| (self.xmin..=self.xmax)
                            .map(|x| if self.cells[&(x, y, z)] {'#'} else {'.'})
                            .collect::<String>()
                        )
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    }
}

impl<T> Grid3d<T>
where T: Default + PartialEq {
    fn pad(&mut self) {
        // Check left (low x)
        if (self.ymin..=self.ymax).any(
            |y| (self.zmin..=self.zmax).any(
                |z| self.cells[&(self.xmin, y, z)] == Default::default()
            )
        ) {
            self.xmin -= 1;
            for y in self.ymin..=self.ymax {
                for z in self.zmin..=self.zmax {
                    self.cells.insert((self.xmin, y, z), Default::default());
                }
            }
        }

        // Check right (high x)
        if (self.ymin..=self.ymax).any(
            |y| (self.zmin..=self.zmax).any(
                |z| self.cells[&(self.xmax, y, z)] == Default::default()
            )
        ) {
            self.xmax += 1;
            for y in self.ymin..=self.ymax {
                for z in self.zmin..=self.zmax {
                    self.cells.insert((self.xmax, y, z), Default::default());
                }
            }
        }

        // Check front (low y)
        if (self.xmin..=self.xmax).any(
            |x| (self.zmin..=self.zmax).any(
                |z| self.cells[&(x, self.ymin, z)] == Default::default()
            )
        ) {
            self.ymin -= 1;
            for x in self.xmin..=self.xmax {
                for z in self.zmin..=self.zmax {
                    self.cells.insert((x, self.ymin, z), Default::default());
                }
            }
        }

        // Check back (high y)
        if (self.xmin..=self.xmax).any(
            |x| (self.zmin..=self.zmax).any(
                |z| self.cells[&(x, self.ymax, z)] == Default::default()
            )
        ) {
            self.ymax += 1;
            for x in self.xmin..=self.xmax {
                for z in self.zmin..=self.zmax {
                    self.cells.insert((x, self.ymax, z), Default::default());
                }
            }
        }

        // Check bottom (low z)
        if (self.xmin..=self.xmax).any(
            |x| (self.ymin..=self.ymax).any(
                |y| self.cells[&(x, y, self.zmin)] == Default::default()
            )
        ) {
            self.zmin -= 1;
            for x in self.xmin..=self.xmax {
                for y in self.ymin..=self.ymax {
                    self.cells.insert((x, y, self.zmin), Default::default());
                }
            }
        }

        // Check top (high z)
        if (self.xmin..=self.xmax).any(
            |x| (self.ymin..=self.ymax).any(
                |y| self.cells[&(x, y, self.zmax)] == Default::default()
            )
        ) {
            self.zmax += 1;
            for x in self.xmin..=self.xmax {
                for y in self.ymin..=self.ymax {
                    self.cells.insert((x, y, self.zmax), Default::default());
                }
            }
        }
    }
}

impl Grid3d<bool> {
    fn step(&mut self) {
        self.pad();

        let old = self.cells.clone();
        for (i, active) in self.cells.iter_mut() {
            // Count neighbours
            let (x, y, z) = i;
            let mut neighbours = 0;
            for x2 in x-1..=x+1 {
                for y2 in y-1..=y+1 {
                    for z2 in z-1..=z+1 {
                        if old.get(&(x2, y2, z2)) == Some(&true) {
                            neighbours += 1;
                        }
                    }
                }
            }

            // Uncount current cell
            if *active {
                neighbours -= 1;
            }

            // Change state according to given rules:
            // - active cells must have 2 or 3 neighbours to stay active
            // - inactive cells become active if exactly 3 neighbours are
            let new_state = match (*active, neighbours) {
                (true, 2..=3) => true,
                (false, 3) => true,
                _ => false,
            };
            *active = new_state;
        }
    }

    fn count_active(&self) -> usize {
        self.cells.values().fold(0, |acc, active| acc + match active {true => 1, false => 0})
    }
}

fn main() -> io::Result<()> {
    // Read file
    let mut grid = Grid3d::from(File::open("inputs/17.txt")?);

    for _ in 0..6 {
        grid.step();
    }
    println!("Part 1: {}", grid.count_active());

    Ok(())
}
