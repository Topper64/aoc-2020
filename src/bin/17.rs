use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct GridND {
    cells: HashSet<Vec<i32>>,
    ndim: u32,
}

impl From<File> for GridND {
    fn from(file: File) -> GridND {
        let reader = BufReader::new(file);

        let mut cells = HashSet::new();
        for (y, line) in reader.lines().map(|line| line.unwrap()).enumerate() {
            let y = y as i32;
            for (x, char) in line.char_indices() {
                let x = x as i32;
                let i = vec![x, y];
                if char == '#' {
                    cells.insert(i);
                }
            }
        }

        GridND {cells, ndim: 2}
    }
}

impl GridND {
    fn with_dims(&self, n: u32) -> GridND {
        let mut cells = HashSet::new();
        for i in self.cells.iter() {
            let mut i = i.clone();
            while i.len() < n as usize {
                i.push(0);
            }
            while i.len() > n as usize {
                i.pop();
            }
            cells.insert(i);
        }

        GridND {cells, ndim: n}
    }

    fn step(&mut self) {
        // Generate all offsets for immediate neighbours
        let mut offsets = Vec::new();
        for mut n in 0..3_usize.pow(self.ndim) {
            let mut offset = Vec::new();
            for _ in 0..self.ndim {
                offset.push(n.rem_euclid(3) as i32 - 1);
                n = n.div_euclid(3);
            }
            if offset.iter().any(|i| i != &0) {
                offsets.push(offset);
            }
        }

        // Count neighbours, by adding 1 to each neighbouring index of each active cell
        let mut neighbours = HashMap::new();
        for index in self.cells.iter() {
            for offset in offsets.iter() {
                let index2: Vec<_> = index.iter()
                    .zip(offset)
                    .map(|(i, j)| i + j)
                    .collect();
                *neighbours.entry(index2).or_insert(0) += 1;
            }
        }

        // Change state according to given rules:
        // - active cells must have 2 or 3 neighbours to stay active
        // - inactive cells become active if exactly 3 neighbours are
        let mut new_cells = HashSet::new();
        for (index, count) in neighbours.iter() {
            let active = self.cells.contains(index);
            let new_state = match (active, count) {
                (true, 2..=3) => true,
                (false, 3) => true,
                _ => false,
            };
            if new_state {
                new_cells.insert(index.to_vec());
            }
        }
        self.cells = new_cells;
    }

    fn count_active(&self) -> usize {
        self.cells.len()
    }
}

fn main() -> io::Result<()> {
    // Read file
    let grid2d = GridND::from(File::open("inputs/17.txt")?);

    let mut grid3d = grid2d.with_dims(3);
    for _ in 0..6 {
        grid3d.step();
    }
    println!("Part 1: {}", grid3d.count_active());

    let mut grid4d = grid2d.with_dims(4);
    for _ in 0..6 {
        grid4d.step();
    }
    println!("Part 2: {}", grid4d.count_active());

    Ok(())
}
