use advent_2024::Grid;
use itertools::Itertools;
use std::ops::{Add, Sub};
use std::{error::Error, fs};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Debug)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.row + other.row, &self.col + other.col)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.row - other.row, &self.col - other.col)
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, col): (usize, usize)) -> Self {
        Self::new(row as i32, col as i32)
    }
}

fn parse_input(input: &str) -> Grid<char> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Grid::new(0, 0, '.');
    }

    let height = lines.len();
    let width = lines[0].len();

    let chars = lines.into_iter().flat_map(|line| line.chars()).collect();

    Grid {
        data: chars,
        width,
        height,
    }
}

fn get_antinodes(a: Position, b: Position, antennas: &Grid<char>, resonant: bool) -> Vec<Position> {
    let delta = a - b;
    let grid_size = antennas.height.max(antennas.width) as i32;

    #[rustfmt::skip]
    let search_k = if resonant {-grid_size..=grid_size} else {1..=1};

    search_k
        .flat_map(|k| {
            let delta_k = Position::new(delta.row * k, delta.col * k);
            [a + delta_k, b - delta_k]
        })
        .filter(|p| antennas.is_within_extents(p.row, p.col))
        .collect()
}

fn find_antinodes_for_freq(antennas: &Grid<char>, freq: char, resonant: bool) -> Vec<Position> {
    let positions: Vec<Position> = antennas
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == freq)
        .map(|(i, _)| (i / antennas.width, i % antennas.width))
        .map(Position::from)
        .collect();

    let antinodes: Vec<Position> = positions
        .iter()
        .tuple_combinations()
        .flat_map(|(&a, &b)| get_antinodes(a, b, &antennas, resonant))
        .unique()
        .collect();

    antinodes
}

fn problem(input: &str, resonant: bool) -> usize {
    let antennas = parse_input(&input);

    antennas
        .iter()
        .unique()
        .filter(|&c| c != &'.' && c != &'\n')
        .map(|&c| c)
        .flat_map(|freq| find_antinodes_for_freq(&antennas, freq, resonant))
        .unique()
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day8.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:}", problem(&input, false));
    println!("Part 2: {:}", problem(&input, true));
    Ok(())
}

#[test]
fn test_part1() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    assert_eq!(problem(&input, false), 14);
}

#[test]
fn test_part2() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    assert_eq!(problem(&input, true), 34);
}
