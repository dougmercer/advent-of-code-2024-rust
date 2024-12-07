use std::ops::{Index, IndexMut};

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

    // pub fn get(&self, x: usize, y: usize) -> Option<&T> {
    //     if x < self.width && y < self.height {
    //         Some(&self.data[y * self.width + x])
    //     } else {
    //         None
    //     }
    // }

    // pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
    //     if x < self.width && y < self.height {
    //         Some(&mut self.data[y * self.width + x])
    //     } else {
    //         None
    //     }
    // }

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
