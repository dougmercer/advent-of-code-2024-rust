use advent_2024::{Graph, Grid};
use std::{error::Error, fs};

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    #[default]
    Empty,
    Corrupted,
    Start,
    End,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Corrupted => '#',
            Cell::Start => 'S',
            Cell::End => 'O',
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
    x: usize,
    y: usize,
    cell: Cell,
}

impl Node {
    fn new(x: usize, y: usize, cell: Cell) -> Self {
        Node { x, y, cell }
    }
}

fn parse_input(
    input: &str,
    width: usize,
    height: usize,
    nbytes: usize,
) -> Result<Grid<Cell>, Box<dyn Error>> {
    let mut grid = input
        .lines()
        .filter_map(|line| {
            let [x, y] = line
                .split(',')
                .map(str::trim)
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<usize>>()[..]
            else {
                return None;
            };
            Some((x, y))
        })
        .take(nbytes)
        .fold(Grid::new(width, height, Cell::Empty), |mut grid, (x, y)| {
            grid[(x, y)] = Cell::Corrupted;
            grid
        });
    grid[(0, 0)] = Cell::Start;
    grid[(width - 1, height - 1)] = Cell::End;

    Ok(grid)
}

fn grid_to_graph(grid: &Grid<Cell>) -> Graph<Node> {
    let mut graph: Graph<Node> = Graph::directed();

    for ((x, y), val) in grid.iter_items() {
        if *val == Cell::Corrupted {
            continue;
        }
        let from = Node::new(x, y, *val);

        for (xn, yn) in grid.cardinal_neighbors(x, y) {
            if let Some(neighbor) = grid.get(xn, yn) {
                if *neighbor != Cell::Corrupted {
                    let to = Node::new(xn, yn, *neighbor);
                    graph.add_edge(from, to);
                }
            }
        }
    }
    graph
}

fn solver(
    input: &str,
    width: usize,
    height: usize,
    nbytes: usize,
) -> Result<usize, Box<dyn Error>> {
    let grid = parse_input(&input, width, height, nbytes)?;
    let graph = grid_to_graph(&grid);
    let start = graph
        .nodes()
        .into_iter()
        .find(|node| node.cell == Cell::Start)
        .ok_or("No start")?
        .clone();
    let end = graph
        .nodes()
        .into_iter()
        .find(|&&node| node.cell == Cell::End)
        .ok_or("No end")?
        .clone();
    let (_, dist) = graph.shortest_path(start, end).ok_or("No shortest path.")?;

    Ok(dist as usize)
}

fn part2(input: &str, width: usize, height: usize) -> Result<&str, Box<dyn Error>> {
    let mut start: usize = 0;
    let mut end: usize = input.lines().count();

    // Dichotomous search
    loop {
        let mid = (start + end) / 2;
        let dist = solver(input, width, height, mid);
        if dist.is_ok() {
            start = mid;
        } else {
            end = mid;
        }
        if start + 1 == end {
            break;
        }
    }
    input.lines().nth(start).ok_or("No line found.".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day18.input")?;
    println!("Part 1: {}", solver(&input, 71, 71, 1024)?);
    println!("Part 2: {}", part2(&input, 71, 71)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn test_part1() {
        assert_eq!(solver(&EXAMPLE, 7, 7, 12).unwrap(), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE, 7, 7).unwrap(), "6,1");
    }
}
