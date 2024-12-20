use advent_2024::{Graph, Grid};
use rayon::prelude::*;
use std::{error::Error, fs};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn minkowski(&self, other: Coordinate) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Start => 'S',
            Cell::End => 'E',
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Cell::Wall),
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            '.' => Ok(Cell::Empty),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    xy: Coordinate,
    cell: Cell,
}

impl Node {
    fn new(xy: Coordinate, cell: Cell) -> Self {
        Node { xy, cell }
    }
}

fn grid_to_graph(grid: &Grid<Cell>) -> Graph<Node> {
    let mut graph: Graph<Node> = Graph::directed();

    for ((x, y), val) in grid.iter_items() {
        if *val == Cell::Wall {
            continue;
        }
        let from = Node::new(Coordinate { x, y }, *val);

        for (xn, yn) in grid.cardinal_neighbors(x, y) {
            if let Some(neighbor) = grid.get(xn, yn) {
                if *neighbor != Cell::Wall {
                    let to = Node::new(Coordinate { x: xn, y: yn }, *neighbor);
                    graph.add_edge(from, to);
                }
            }
        }
    }
    graph
}

fn find_thing(grid: &Grid<Cell>, query: Cell) -> Result<(usize, usize), Box<dyn Error>> {
    grid.iter()
        .enumerate()
        .find(|(_, &cell_type)| cell_type == query)
        .map(|(idx, _)| grid.idx_to_xy(idx))
        .ok_or_else(|| "Not found".into())
}

fn solver(
    input: &str,
    time_saved: usize,
    max_cheat_duration: usize,
) -> Result<usize, Box<dyn Error>> {
    let grid = Grid::parse_str(input, Cell::try_from, Cell::default())?;
    let graph = grid_to_graph(&grid);

    let (sx, sy) = find_thing(&grid, Cell::Start)?;
    let start = Node {
        xy: Coordinate { x: sx, y: sy },
        cell: Cell::Start,
    };

    let path = graph.bfs(start).map(|x| x.xy).collect::<Vec<_>>();

    Ok(path
        .par_iter()
        .enumerate()
        .take(path.len() - time_saved)
        .map(|(i, &from)| {
            path.iter()
                .enumerate()
                .skip(i + time_saved)
                .filter(|(j, &to)| {
                    let d = from.minkowski(to);
                    d <= max_cheat_duration && (j - i - d) >= time_saved
                })
                .count()
        })
        .sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day20.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {}", solver(&input, 100, 2)?);
    println!("Part 2: {:?}", solver(&input, 100, 20)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn test_example1() {
        assert_eq!(solver(&EXAMPLE, 19, 2).unwrap(), 5);
        assert_eq!(solver(&EXAMPLE, 20, 2).unwrap(), 5);
        assert_eq!(solver(&EXAMPLE, 21, 2).unwrap(), 4);
        assert_eq!(solver(&EXAMPLE, 35, 2).unwrap(), 4);
        assert_eq!(solver(&EXAMPLE, 36, 2).unwrap(), 4);
        assert_eq!(solver(&EXAMPLE, 37, 2).unwrap(), 3);
        assert_eq!(solver(&EXAMPLE, 38, 2).unwrap(), 3);
        assert_eq!(solver(&EXAMPLE, 39, 2).unwrap(), 2);
        assert_eq!(solver(&EXAMPLE, 40, 2).unwrap(), 2);
        assert_eq!(solver(&EXAMPLE, 41, 2).unwrap(), 1);
        assert_eq!(solver(&EXAMPLE, 64, 2).unwrap(), 1);
        assert_eq!(solver(&EXAMPLE, 65, 2).unwrap(), 0);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solver(&EXAMPLE, 50, 20).unwrap(), 285);
        assert_eq!(solver(&EXAMPLE, 52, 20).unwrap(), 253);
        assert_eq!(solver(&EXAMPLE, 54, 20).unwrap(), 222);
        assert_eq!(solver(&EXAMPLE, 56, 20).unwrap(), 193);
        assert_eq!(solver(&EXAMPLE, 58, 20).unwrap(), 154);
        assert_eq!(solver(&EXAMPLE, 60, 20).unwrap(), 129);
        assert_eq!(solver(&EXAMPLE, 62, 20).unwrap(), 106);
        assert_eq!(solver(&EXAMPLE, 64, 20).unwrap(), 86);
        assert_eq!(solver(&EXAMPLE, 66, 20).unwrap(), 67);
        assert_eq!(solver(&EXAMPLE, 68, 20).unwrap(), 55);
        assert_eq!(solver(&EXAMPLE, 70, 20).unwrap(), 41);
        assert_eq!(solver(&EXAMPLE, 72, 20).unwrap(), 29);
        assert_eq!(solver(&EXAMPLE, 74, 20).unwrap(), 7);
        assert_eq!(solver(&EXAMPLE, 76, 20).unwrap(), 3);
    }
}
