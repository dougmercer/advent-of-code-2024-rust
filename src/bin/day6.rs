use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::{error::Error, fs};

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
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

struct Map {
    occupancy: Grid<bool>,
    visited: Grid<bool>,
    // visited_by_direction: HashMap<Direction, Grid<bool>>
}

impl Map {
    fn from_str(input: &str) -> Map {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        // Create a grid and fill it
        let mut occupancy = Grid::new(width, height, false);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                occupancy[(x, y)] = ch == '#';
            }
        }

        let visited = Grid::new(occupancy.width, occupancy.height, false);
        // let visited_by_direction: HashMap<Direction, Grid<bool>> = Direction::all().iter().fold(
        //     HashMap::new(), |mut map, &x| {
        //     map.entry(x);
        //     map
        // });

        // Map { occupancy, visited, visited_by_direction }

        Map { occupancy, visited }
    }

    fn width(&self) -> usize {
        self.occupancy.width
    }
    fn height(&self) -> usize {
        self.occupancy.height
    }
    fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.occupancy[(x, y)]
    }
    fn is_within_extents(&self, x: usize, y: usize) -> bool {
        self.occupancy.is_within_extents(x as i32, y as i32)
    }
    fn visit(&mut self, x: usize, y: usize) {
        // , direction: Direction
        self.visited[(x, y)] = true;
        // self.visited_by_direction[direction][(x,y)] = true;
    }
}

// #[derive(Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(ch: char) -> Result<Direction, &'static str> {
        match ch {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("Invalid direction character"),
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    // Get all possible directions
    fn all() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            }
        )
    }
}
struct Guard<'a> {
    i: i32,
    j: i32,
    map: &'a mut Map,
    direction: Direction,
}

impl<'a> Guard<'a> {
    fn new(i: i32, j: i32, map: &'a mut Map, direction: Direction) -> Guard {
        // map.visit(i as usize, j as usize, direction);
        map.visit(i as usize, j as usize);
        Guard {
            i,
            j,
            map,
            direction,
        }
    }

    fn _get_next_candidate_position(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.i, self.j - 1),
            Direction::Right => (self.i + 1, self.j),
            Direction::Down => (self.i, self.j + 1),
            Direction::Left => (self.i - 1, self.j),
        }
    }

    // return true if still in room
    fn step(&mut self) -> bool {
        loop {
            let (x, y) = self._get_next_candidate_position();
            if !self.map.is_within_extents(x as usize, y as usize) {
                self.i = x;
                self.j = y;
                return false;
            }
            if self.map.is_occupied(x as usize, y as usize) {
                self.direction = self.direction.turn_right();
                continue;
            } else {
                self.i = x;
                self.j = y;
                self.map.visit(x as usize, y as usize);
                break;
            }
        }
        true
    }
}

impl std::fmt::Debug for Guard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.map.occupancy.height {
            for x in 0..self.map.occupancy.width {
                if x as i32 == self.i && y as i32 == self.j {
                    write!(f, "{}", self.direction)?;
                } else {
                    write!(f, "{}", if self.map.occupancy[(x, y)] { '#' } else { '.' })?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.occupancy.height {
            for x in 0..self.occupancy.width {
                if self.is_occupied(x, y) {
                    write!(f, "{}", "#")?;
                } else {
                    write!(f, "{}", if self.visited[(x, y)] { 'x' } else { '.' })?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Guard<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Guard at ({}, {}) facing {}",
            self.i, self.j, self.direction
        )
    }
}

fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let mut map = Map::from_str(&contents);

    // Find the guard's starting position
    let mut guard_pos = None;
    let mut direction: Direction = Direction::Down;
    for (y, line) in contents.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if "^v<>".contains(ch) {
                guard_pos = Some((x as i32, y as i32));
                direction = Direction::from_char(ch)?;
                break;
            }
        }
    }

    if let Some((i, j)) = guard_pos {
        let mut guard = Guard::new(i, j, &mut map, direction);
        println!("{}", guard);
        let mut is_in_room: bool = true;
        while is_in_room {
            is_in_room = guard.step();
            // println!("{:?}", guard);
        }
        println!("{:?}", map);
        return Ok(map.visited.iter().filter(|&x| *x).count());
    }
    Ok(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day6.input";
    println!("{:?}", part1(path)?);

    Ok(())
}
