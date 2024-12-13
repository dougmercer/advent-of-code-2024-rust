use advent_2024::Grid;
use bitflags::bitflags;
use std::{error::Error, fs};

bitflags! {
    #[derive(Clone, Copy, Default)]
    struct VisitFlags: u8 {
        const NONE  = 0b0000;
        const UP    = 0b0001;
        const DOWN  = 0b0010;
        const LEFT  = 0b0100;
        const RIGHT = 0b1000;
    }
}
#[derive(Copy, Clone)]
struct GuardState {
    position: (i32, i32),
    direction: Direction,
}

#[derive(Clone)]
struct Map {
    occupancy: Grid<bool>,
    visited: Grid<VisitFlags>,
}

impl Map {
    fn from_str(input: &str) -> (Map, Option<GuardState>) {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut occupancy = Grid::new(width, height, false);
        let mut guard_start: Option<GuardState> = None;

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if "^v<>".contains(ch) {
                    guard_start = Some(GuardState {
                        position: (x as i32, y as i32),
                        direction: Direction::from_char(ch).unwrap(),
                    });
                } else {
                    occupancy[(x, y)] = ch == '#';
                }
            }
        }

        let visited = Grid::new(occupancy.width, occupancy.height, VisitFlags::empty());

        (Map { occupancy, visited }, guard_start)
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
    fn is_within_extents(&self, x: i32, y: i32) -> bool {
        self.occupancy.is_within_extents(x, y)
    }
    fn visit(&mut self, x: usize, y: usize, dir: &Direction) {
        self.visited[(x, y)] |= dir.as_visit_flag();
    }

    fn is_visited(&self, x: usize, y: usize) -> bool {
        !self.visited[(x, y)].is_empty()
    }

    fn is_visited_in_direction(&self, x: usize, y: usize, dir: &Direction) -> bool {
        self.visited[(x, y)].contains(dir.as_visit_flag())
    }

    fn add_obstacle(&mut self, x: usize, y: usize) {
        self.occupancy[(x, y)] = true;
    }

    fn debug_loop_points(&self, guard_pos: (i32, i32), loop_points: &[(usize, usize)]) -> String {
        let mut output = String::new();
        output.push('\n');

        for y in 0..self.height() {
            for x in 0..self.width() {
                if (x as i32, y as i32) == guard_pos {
                    output.push('G');
                } else if self.is_occupied(x, y) {
                    output.push('#');
                } else if loop_points.contains(&(x, y)) {
                    output.push('O');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }
        output
    }
}

#[derive(Clone, Copy)]
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

    fn as_visit_flag(&self) -> VisitFlags {
        match self {
            Direction::Up => VisitFlags::UP,
            Direction::Down => VisitFlags::DOWN,
            Direction::Left => VisitFlags::LEFT,
            Direction::Right => VisitFlags::RIGHT,
        }
    }

    // fn all() -> &'static [Direction] {
    //     &[
    //         Direction::Up,
    //         Direction::Down,
    //         Direction::Left,
    //         Direction::Right,
    //     ]
    // }
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
        map.visit(i as usize, j as usize, &direction);
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

    fn peek_step(&mut self) -> (i32, i32, Direction) {
        loop {
            let (x, y) = self._get_next_candidate_position();
            if !self.map.is_within_extents(x, y) {
                // Leaving the extents of the room
                return (x, y, self.direction);
            }
            if self.map.is_occupied(x as usize, y as usize) {
                return (x, y, self.direction.turn_right());
            } else {
                return (x, y, self.direction);
            }
        }
    }

    fn step(&mut self) -> bool {
        loop {
            let (x, y) = self._get_next_candidate_position();
            if !self.map.is_within_extents(x, y) {
                // Leaving the extents of the room
                self.i = x;
                self.j = y;
                return false;
            }
            if self.map.is_occupied(x as usize, y as usize) {
                self.direction = self.direction.turn_right();
                return true;
            } else {
                self.i = x;
                self.j = y;
                self.map.visit(x as usize, y as usize, &self.direction);
                return true;
            }
        }
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
                    write!(f, "{}", if self.is_visited(x, y) { 'x' } else { '.' })?;
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

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let (mut map, guard_start) = Map::from_str(&input);

    if let Some(start) = guard_start {
        let mut guard = Guard::new(
            start.position.0,
            start.position.1,
            &mut map,
            start.direction,
        );
        // println!("{}", guard);
        let mut is_in_room: bool = true;
        while is_in_room {
            is_in_room = guard.step();
            // println!("{:?}", guard);
        }
        // println!("{:?}", map);
        return Ok(map.visited.iter().filter(|&x| !x.is_empty()).count());
    }
    Ok(0)
}

fn check_if_would_loop_if_obstacle(
    x: i32,
    y: i32,
    map: &Map,
    guard_start: &GuardState,
) -> Option<(usize, usize)> {
    let mut map = map.clone();
    map.add_obstacle(x as usize, y as usize);
    let mut guard = Guard::new(
        guard_start.position.0,
        guard_start.position.1,
        &mut map,
        guard_start.direction,
    );
    let mut x_next: i32;
    let mut y_next: i32;
    let mut dir_next: Direction;
    loop {
        (x_next, y_next, dir_next) = guard.peek_step();
        if !guard.map.is_within_extents(x_next, y_next) {
            return None;
        }

        if guard
            .map
            .is_visited_in_direction(x_next as usize, y_next as usize, &dir_next)
        {
            return Some((x_next as usize, y_next as usize));
        }
        guard.step();
    }
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let (map, guard_start) = Map::from_str(&input);

    if let Some(start) = guard_start {
        let loop_points: Vec<(usize, usize)> = (0..map.width())
            .flat_map(|x| (0..map.height()).map(move |y| (x, y)))
            .filter(|(x, y)| (start.position.0, start.position.1) != (*x as i32, *y as i32))
            .filter(|(x, y)| !map.is_occupied(*x, *y))
            .filter_map(|(x, y)| check_if_would_loop_if_obstacle(x as i32, y as i32, &map, &start))
            .collect();

        // println!("{}", map.debug_loop_points(start.position, &loop_points));
        Ok(loop_points.len())
    } else {
        Ok(0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day6.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    assert_eq!(part1(&input).unwrap(), 41);
}

#[test]
fn test_part2() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    assert_eq!(part2(&input).unwrap(), 6);
}
