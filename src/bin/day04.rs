use itertools::iproduct;
use itertools::multizip;
use std::{error::Error, fs};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UR,
    UL,
    DR,
    DL,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::UP => (0, -3),
            Direction::DOWN => (0, 3),
            Direction::LEFT => (-3, 0),
            Direction::RIGHT => (3, 0),
            Direction::UR => (3, -3),
            Direction::UL => (-3, -3),
            Direction::DR => (3, 3),
            Direction::DL => (-3, 3),
        }
    }
    // Get all possible directions
    fn all() -> &'static [Direction] {
        &[
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
            Direction::UR,
            Direction::UL,
            Direction::DR,
            Direction::DL,
        ]
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_in_bounds<T>(grid: &[Vec<T>], row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && row < grid.len() as i32 && col < grid[0].len() as i32
}

fn search_xmas(grid: &[Vec<char>], i: usize, j: usize, direction: Direction) -> bool {
    let (x, y) = direction.offset();

    if !is_in_bounds(grid, (i as i32) + x, (j as i32) + y) {
        return false;
    }

    let row_step = x / 3;
    let col_step = y / 3;
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    for k in 0..4 {
        let row_index = ((i as i32) + k * row_step) as usize;
        let col_index = ((j as i32) + k * col_step) as usize;

        if grid[row_index][col_index] != XMAS[k as usize] {
            return false;
        }
    }

    true
}

fn search_double_mas(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    if !is_in_bounds(grid, (i as i32) + -1, (j as i32) + -1)
        || !is_in_bounds(grid, (i as i32) + 1, (j as i32) + 1)
    {
        return false;
    }

    const MAS: [char; 3] = ['M', 'A', 'S'];

    // Check \ direction
    // Either
    //      row/col  -1 0 1 / 1 0 -1
    // or   row/col  -1 0 1 / 1 0 -1
    // Should match   M A S
    let found_diagonal = [-1, 1].iter().any(|&sign| {
        multizip((0..3, -1..2, -1..2)).all(|(k, i_offset, j_offset)| {
            let row_index = ((i as i32) + sign * i_offset) as usize;
            let col_index = ((j as i32) + sign * j_offset) as usize;
            grid[row_index][col_index] == MAS[k as usize]
        })
    });

    // Check other direction /
    // Either
    //      row/col  -1 0  1 /  1 0 -1
    // or   row/col   1 0 -1 / -1 0  1
    // Should match   M A S
    let found_antidiagonal = [-1, 1].iter().any(|&sign| {
        multizip((0..3, -1..2, (-1..2).rev())).all(|(k, i_offset, j_offset)| {
            let row_index = ((i as i32) + sign * i_offset) as usize;
            let col_index = ((j as i32) + sign * j_offset) as usize;
            grid[row_index][col_index] == MAS[k as usize]
        })
    });

    found_diagonal && found_antidiagonal
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    iproduct!(0..grid.len(), 0..grid[0].len(), Direction::all())
        .filter(|&(i, j, direction)| search_xmas(&grid, i, j, *direction))
        .count()
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(i, j)| search_double_mas(&grid, i, j))
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day4.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = [
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ]
    .join("\n");
    assert_eq!(part1(&input), 18);
}

#[test]
fn test_part2() {
    let input = [
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ]
    .join("\n");
    assert_eq!(part2(&input), 9);
}
