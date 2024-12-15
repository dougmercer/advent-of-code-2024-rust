use advent_2024::Grid;
use std::{error::Error, fs};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Cell {
    #[default]
    Empty,
    Robot,
    Box,
    WideBoxLeft,
    WideBoxRight,
    Wall,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Wall => '#',
            Cell::Robot => '@',
            Cell::Box => 'O',
            Cell::Empty => '.',
            Cell::WideBoxLeft => '[',
            Cell::WideBoxRight => ']',
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Cell::Wall),
            '@' => Ok(Cell::Robot),
            'O' => Ok(Cell::Box),
            '.' => Ok(Cell::Empty),
            '[' => Ok(Cell::WideBoxLeft),
            ']' => Ok(Cell::WideBoxRight),
            _ => Err(format!("Invalid character: {}", c)),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

type ParserOutput = Result<(Grid<Cell>, Vec<Direction>), Box<dyn Error>>;

fn parse_input(input: &str, widen: bool) -> ParserOutput {
    let (raw_room_str, commands_str) = input.split_once("\n\n").unwrap_or((&input, ""));

    fn widen_room(room_str: &str) -> String {
        room_str
            .chars()
            .flat_map(|c| match c {
                '#' => vec!['#', '#'],
                'O' => vec!['[', ']'],
                '.' => vec!['.', '.'],
                '@' => vec!['@', '.'],
                c => vec![c],
            })
            .collect()
    }

    let room_str = if widen {
        widen_room(raw_room_str)
    } else {
        raw_room_str.to_string()
    };
    let grid: Grid<Cell> = Grid::parse_str(&room_str, Cell::try_from, Cell::default())?;

    let commands: Vec<Direction> = commands_str
        .chars()
        .filter_map(Direction::from_char)
        .collect();

    Ok((grid, commands))
}

fn check_wide_vertical_move(
    grid: &Grid<Cell>,
    left: (usize, usize),
    right: (usize, usize),
    dir: Direction,
) -> Result<bool, Box<dyn Error>> {
    let next_left = (
        (left.0 as i32 + dir.offset().0) as usize,
        (left.1 as i32 + dir.offset().1) as usize,
    );
    let next_right = (
        (right.0 as i32 + dir.offset().0) as usize,
        (right.1 as i32 + dir.offset().1) as usize,
    );

    match (grid[next_left], grid[next_right]) {
        (Cell::Empty, Cell::Empty) => Ok(true),
        (Cell::Wall, _) | (_, Cell::Wall) => Ok(false),
        _ => Ok(can_move(grid, left, dir)? && can_move(grid, right, dir)?),
    }
}

fn check_wide_box_move(
    grid: &Grid<Cell>,
    pos: (usize, usize),
    dir: Direction,
) -> Result<bool, Box<dyn Error>> {
    match (grid[pos], dir) {
        (Cell::WideBoxLeft, Direction::Up | Direction::Down) => {
            let right_pos = (pos.0 + 1, pos.1);
            if grid[right_pos] != Cell::WideBoxRight {
                return Err("Invalid wide box state".into());
            }
            check_wide_vertical_move(grid, pos, right_pos, dir)
        }
        (Cell::WideBoxRight, Direction::Up | Direction::Down) => {
            let left_pos = (pos.0 - 1, pos.1);
            if grid[left_pos] != Cell::WideBoxLeft {
                return Err("Invalid wide box state".into());
            }
            check_wide_vertical_move(grid, left_pos, pos, dir)
        }
        _ => can_move(grid, pos, dir),
    }
}

fn can_move(
    grid: &Grid<Cell>,
    pos: (usize, usize),
    dir: Direction,
) -> Result<bool, Box<dyn Error>> {
    let next_pos = (
        (pos.0 as i32 + dir.offset().0) as usize,
        (pos.1 as i32 + dir.offset().1) as usize,
    );

    match grid[next_pos] {
        Cell::Empty => Ok(true),
        Cell::Wall => Ok(false),
        Cell::Box => can_move(grid, next_pos, dir),
        Cell::WideBoxLeft | Cell::WideBoxRight => check_wide_box_move(grid, next_pos, dir),
        Cell::Robot => Err("Invalid state: encountered another robot".into()),
    }
}

fn handle_box_push(
    grid: &mut Grid<Cell>,
    pos: (usize, usize),
    dir: Direction,
) -> Result<(), Box<dyn Error>> {
    match (grid[pos], dir) {
        (Cell::WideBoxLeft, Direction::Up | Direction::Down) => {
            let right_pos = (pos.0 + 1, pos.1);
            push(grid, pos, dir)?;
            push(grid, right_pos, dir)?;
        }
        (Cell::WideBoxRight, Direction::Up | Direction::Down) => {
            let left_pos = (pos.0 - 1, pos.1);
            push(grid, left_pos, dir)?;
            push(grid, pos, dir)?;
        }
        _ => push(grid, pos, dir)?,
    }
    Ok(())
}

fn push(grid: &mut Grid<Cell>, pos: (usize, usize), dir: Direction) -> Result<(), Box<dyn Error>> {
    if !can_move(grid, pos, dir)? {
        return Ok(());
    }

    let next_pos = (
        (pos.0 as i32 + dir.offset().0) as usize,
        (pos.1 as i32 + dir.offset().1) as usize,
    );

    match grid[next_pos] {
        Cell::Empty => {
            grid.swap(pos, next_pos);
        }
        Cell::Box | Cell::WideBoxLeft | Cell::WideBoxRight => {
            handle_box_push(grid, next_pos, dir)?;
            grid.swap(pos, next_pos);
        }
        _ => {}
    }
    Ok(())
}

fn find_robot(grid: &Grid<Cell>) -> Result<(usize, usize), Box<dyn Error>> {
    grid.iter()
        .enumerate()
        .find(|(_, &cell)| cell == Cell::Robot)
        .map(|(idx, _)| grid.idx_to_xy(idx))
        .ok_or_else(|| "No robot found".into())
}

fn compute_gps(grid: &Grid<Cell>) -> usize {
    grid.iter()
        .enumerate()
        .filter_map(|(idx, cell)| match cell {
            Cell::WideBoxLeft | Cell::Box => Some(grid.idx_to_xy(idx)),
            _ => None,
        })
        .map(|(x, y)| y * 100 + x)
        .sum()
}

fn solver(input: &str, parser: fn(&str) -> ParserOutput) -> Result<usize, Box<dyn Error>> {
    let (mut grid, commands) = parser(&input)?;
    for command in commands {
        let robot_xy = find_robot(&grid)?;
        let _ = push(&mut grid, robot_xy, command);
    }

    Ok(compute_gps(&grid))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day15.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", solver(&input, |x| parse_input(x, false))?);
    println!("Part 2: {:?}", solver(&input, |x| parse_input(x, true))?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const LARGE_EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn test_small_example() {
        assert_eq!(
            solver(&SMALL_EXAMPLE, |x| parse_input(x, false)).unwrap(),
            2028
        );
    }

    #[test]
    fn test_large_example() {
        assert_eq!(
            solver(&LARGE_EXAMPLE, |x| parse_input(x, false)).unwrap(),
            10092
        );
    }

    #[test]
    fn test_large_wide_example() {
        assert_eq!(
            solver(&LARGE_EXAMPLE, |x| parse_input(x, true)).unwrap(),
            9021
        );
    }
}
