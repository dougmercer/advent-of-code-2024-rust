use advent_2024::graph::Dijkstra;
use advent_2024::{Graph, Grid};
use itertools::{iproduct, Itertools};
use std::{error::Error, fs};

#[derive(Default, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
enum CellType {
    #[default]
    Empty,
    Start,
    End,
    Wall,
}

impl CellType {
    fn to_char(&self) -> char {
        match self {
            CellType::Wall => '#',
            CellType::Start => 'S',
            CellType::End => 'O',
            CellType::Empty => '.',
        }
    }
}

impl TryFrom<char> for CellType {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(CellType::Wall),
            'S' => Ok(CellType::Start),
            'E' => Ok(CellType::End),
            '.' => Ok(CellType::Empty),
            _ => Err(format!("Invalid character: {}", c)),
        }
    }
}

impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl std::fmt::Debug for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Any,
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Any => 'O',
        }
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Any => (0, 0), // Really we shouldn't use this, but I don't feel like adding Err
        }
    }

    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Any => Direction::Any,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Any => Direction::Any,
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

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Cell {
    cell_type: CellType,
    direction: Direction,
    xy: (usize, usize),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.cell_type, self.direction, self.xy)
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?})",
            self.cell_type, self.direction, self.xy
        )
    }
}

const ROTATION_COST: usize = 1000;
const STEP_COST: usize = 1;
const STARTING_DIRECTION: Direction = Direction::Right;

fn add_edges(
    mut graph: Graph<Cell, usize>,
    grid: &Grid<CellType>,
    x: usize,
    y: usize,
    direction: Direction,
) -> Graph<Cell, usize> {
    let cell_type = grid[(x, y)];
    let from: Cell = Cell {
        cell_type: cell_type,
        direction: if cell_type == CellType::End {
            Direction::Any
        } else {
            direction
        },
        xy: (x, y),
    };

    graph.add_edge_weighted(
        from,
        Cell {
            cell_type: cell_type,
            direction: direction.turn_right(),
            xy: (x, y),
        },
        ROTATION_COST,
    );
    graph.add_edge_weighted(
        from,
        Cell {
            cell_type: cell_type,
            direction: direction.turn_left(),
            xy: (x, y),
        },
        ROTATION_COST,
    );
    let offset = direction.offset();
    let step_xy = (x as i32 + offset.0, y as i32 + offset.1);
    if grid.is_within_extents(step_xy.0, step_xy.1) {
        let next_xy = (step_xy.0 as usize, step_xy.1 as usize);
        if grid[next_xy] == CellType::Wall {
            return graph;
        }
        let next_cell_type = grid[next_xy];
        let next_direction = if next_cell_type == CellType::End {
            Direction::Any
        } else {
            direction
        };
        graph.add_edge_weighted(
            from,
            Cell {
                cell_type: next_cell_type,
                direction: next_direction,
                xy: next_xy,
            },
            STEP_COST,
        );
    }
    graph
}

fn find_thing(grid: &Grid<CellType>, query: CellType) -> Result<(usize, usize), Box<dyn Error>> {
    grid.iter()
        .enumerate()
        .find(|(_, &cell_type)| cell_type == query)
        .map(|(idx, _)| grid.idx_to_xy(idx))
        .ok_or_else(|| "Not found".into())
}

fn solver1(input: &str) -> Result<usize, Box<dyn Error>> {
    let grid: Grid<CellType> = Grid::parse_str(input, CellType::try_from, CellType::default())?;
    // println!("{grid}");
    let g: Graph<Cell, usize> = iproduct!(0..grid.width, 0..grid.height, Direction::all())
        .fold(Graph::directed(), |graph, (x, y, direction)| {
            add_edges(graph, &grid, x, y, direction)
        });

    // Find start
    let start_xy = find_thing(&grid, CellType::Start)?;
    let end_xy = find_thing(&grid, CellType::End)?;

    let start = Cell {
        cell_type: CellType::Start,
        direction: STARTING_DIRECTION,
        xy: start_xy,
    };

    let end = Cell {
        cell_type: CellType::End,
        direction: Direction::Any,
        xy: end_xy,
    };

    let mut dijkstra = Dijkstra::new(&g, start);
    let (_, distance) = dijkstra.shortest_path(&end).unwrap();

    Ok(distance)
}

fn solver2(input: &str) -> Result<usize, Box<dyn Error>> {
    let grid: Grid<CellType> = Grid::parse_str(input, CellType::try_from, CellType::default())?;
    // println!("{grid}");
    let g: Graph<Cell, usize> = iproduct!(0..grid.width, 0..grid.height, Direction::all())
        .fold(Graph::directed(), |graph, (x, y, direction)| {
            add_edges(graph, &grid, x, y, direction)
        });

    // Find start
    let start_xy = find_thing(&grid, CellType::Start)?;
    let end_xy = find_thing(&grid, CellType::End)?;

    let start = Cell {
        cell_type: CellType::Start,
        direction: STARTING_DIRECTION,
        xy: start_xy,
    };

    let end = Cell {
        cell_type: CellType::End,
        direction: Direction::Any,
        xy: end_xy,
    };

    let mut dijkstra = Dijkstra::new(&g, start);
    let (paths, _) = dijkstra.all_shortest_paths(&end).unwrap();

    Ok(paths
        .into_iter()
        .flatten()
        .map(|cell| cell.xy)
        .unique()
        .count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day16.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", solver1(&input)?);
    println!("Part 2: {:?}", solver2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const EXAMPLE2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn test_example1() {
        assert_eq!(solver1(&EXAMPLE1).unwrap(), 7036);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solver1(&EXAMPLE2).unwrap(), 11048);
    }
}
