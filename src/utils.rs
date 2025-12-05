use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            width,
            height,
            cells: vec![T::default(); width * height],
        }
    }
    fn coordinate_to_index(&self, width: usize, height: usize) -> usize {
        height * self.width + width
    }

    pub fn get(&self, width: usize, height: usize) -> T
    where
        T: Clone + Copy,
    {
        self.cells[self.coordinate_to_index(width, height)]
    }

    pub fn set(&mut self, width: usize, height: usize, value: T) {
        let index = self.coordinate_to_index(width, height);
        self.cells[index] = value
    }

    pub fn neighbors4(&self, x: usize, y: usize) -> Vec<((usize, usize), &T)> {
        let mut result = Vec::with_capacity(4);
        let dirs = [
            (0isize, -1isize), // up
            (0, 1),            // down
            (-1, 0),           // left
            (1, 0),            // right
        ];

        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                let ux = nx as usize;
                let uy = ny as usize;
                result.push(((ux, uy), &self.cells[self.coordinate_to_index(ux, uy)]));
            }
        }

        result
    }

    pub fn neighbors8(&self, x: usize, y: usize) -> Vec<((usize, usize), &T)> {
        let mut result = Vec::with_capacity(8);
        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    result.push(((ux, uy), &self.cells[self.coordinate_to_index(ux, uy)]));
                }
            }
        }
        result
    }

    pub fn row(&self, y: usize) -> Vec<((usize, usize), &T)> {
        let mut result = Vec::with_capacity(self.width);
        for x in 0..self.width {
            result.push(((x, y), &self.cells[self.coordinate_to_index(x, y)]));
        }
        result
    }

    pub fn column(&self, x: usize) -> Vec<((usize, usize), &T)> {
        let mut result = Vec::with_capacity(self.height);
        for y in 0..self.height {
            result.push(((x, y), &self.cells[self.coordinate_to_index(x, y)]));
        }
        result
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.cells[self.coordinate_to_index(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T>
where
    T: FromStr,
{
    pub fn from_ascii(input: &str) -> Result<Self, T::Err> {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map(|l| l.len()).unwrap_or(0);

        let mut cells = Vec::with_capacity(width * height);

        for line in lines {
            assert!(line.len() == width, "Grid rows must have equal length");

            for ch in line.chars() {
                let value = T::from_str(&ch.to_string())?;
                cells.push(value);
            }
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = ((usize, usize), &'a T);
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            index: 0,
        }
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.cells.len() {
            return None;
        }

        let idx = self.index;
        self.index += 1;

        let x = idx % self.grid.width;
        let y = idx / self.grid.width;

        Some(((x, y), &self.grid.cells[idx]))
    }
}
