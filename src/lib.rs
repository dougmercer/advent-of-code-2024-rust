use std::iter::successors;
use std::ops::{Index, IndexMut};

pub mod graph;
pub use graph::Graph;

#[derive(Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Grid {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn parse_str<F>(input: &str, convert: F, default: T) -> Result<Self, String>
    where
        F: Fn(char) -> Result<T, String>,
        T: Clone,
    {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Ok(Grid::new(0, 0, default));
        }

        let width = lines[0].len();
        if lines.iter().any(|line| line.len() != width) {
            return Err("Found line with incorrect width".to_string());
        }

        let data = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(convert)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Grid {
            data,
            width,
            height: lines.len(),
        })
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_idx(&self, idx: usize) -> Option<&T> {
        if idx < self.data.len() {
            Some(&self.data[idx])
        } else {
            None
        }
    }

    pub fn get_idx_mut(&mut self, idx: usize) -> Option<&T> {
        if idx < self.data.len() {
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    pub fn iter_row(&self, y: usize) -> impl Iterator<Item = &T> {
        self.data[y * self.width..(y + 1) * self.width].iter()
    }

    pub fn iter_col(&self, x: usize) -> impl Iterator<Item = &T> {
        self.data.iter().skip(x).step_by(self.width)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn is_within_extents(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < (self.width as i32) && y >= 0 && y < (self.height as i32)
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                if new_x >= 0
                    && new_x < self.width as i32
                    && new_y >= 0
                    && new_y < self.height as i32
                {
                    result.push((new_x as usize, new_y as usize));
                }
            }
        }
        result
    }

    pub fn cardinal_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
                result.push((new_x as usize, new_y as usize));
            }
        }
        result
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ax, ay) = a;
        let (bx, by) = b;
        self.data.swap(ay * self.width + ax, by * self.width + bx);
    }

    pub fn idx_to_xy(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.width;
        let y = (idx - x) / self.width;
        (x, y)
    }
}

// Implement Index/IndexMut for convenient access with []
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y > 0 {
                writeln!(f)?;
            }
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
        }
        Ok(())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid {{")?;
        writeln!(f, "  width: {}", self.width)?;
        writeln!(f, "  height: {}", self.height)?;
        writeln!(f, "  data:")?;
        for y in 0..self.height {
            write!(f, "    ")?;
            for x in 0..self.width {
                write!(f, "{:?} ", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

pub fn digits(n: u64) -> u32 {
    successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count() as u32
}
