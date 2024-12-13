use advent_2024::Grid;
use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::{error::Error, fs};

const SUMMIT_HEIGHT: u8 = 9;
const TRAILHEAD_HEIGHT: u8 = 0;

fn read_input(path: &str) -> Result<Grid<u8>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Ok(Grid::new(0, 0, 0));
    }

    let height = lines.len();
    let width = lines[0].len();

    let data = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    Ok(Grid {
        data,
        width,
        height,
    })
}
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Debug)]
struct Node {
    xy: (usize, usize),
    value: u8,
}

type Graph = HashMap<Node, Vec<Node>>;

fn to_graph(values: &Grid<u8>) -> Graph {
    let mut graph: Graph = HashMap::new();

    for x in 0..values.width {
        for y in 0..values.height {
            let height = values[(x, y)];
            let mut neighbors: Vec<Node> = Vec::new();

            for i in -1..=1i32 {
                for j in -1..=1i32 {
                    if i.abs() + j.abs() == 2 {
                        continue;
                    }

                    let xn = (x as i32) + i;
                    let yn = (y as i32) + j;
                    if !values.is_within_extents(xn, yn) {
                        continue;
                    }
                    let vn = values[(xn as usize, yn as usize)];
                    if height + 1 == vn {
                        neighbors.push(Node {
                            xy: (xn as usize, yn as usize),
                            value: vn,
                        });
                    }
                }
            }
            graph.insert(
                Node {
                    xy: (x, y),
                    value: height,
                },
                neighbors,
            );
        }
    }
    graph
}

// as_rating=false counts reachable summits, true counts paths to summits
fn rate_trailhead(start: &Node, graph: &Graph, as_rating: bool) -> usize {
    let mut summits: usize = 0;
    let mut queue: VecDeque<&Node> = VecDeque::new();
    let mut explored: HashSet<&Node> = HashSet::new();

    queue.push_back(start);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if !as_rating && explored.contains(current) {
            continue;
        }
        explored.insert(current);

        if current.value == SUMMIT_HEIGHT {
            summits += 1;
            continue;
        }

        let edges = graph.get(&current).unwrap();
        for next in edges {
            if !explored.contains(next) {
                queue.push_back(next);
            }
        }
    }

    summits
}

fn problem(path: &str, as_rating: bool) -> Result<usize, Box<dyn Error>> {
    let values = read_input(path)?;
    let graph = to_graph(&values);

    // Get list of trailheads (0s)
    let zeros: Vec<(usize, usize)> = iproduct!(0..values.height, 0..values.width)
        .filter(|(x, y)| values[(*x, *y)] == TRAILHEAD_HEIGHT)
        .collect();

    // Evaluate each trailhead
    Ok(zeros
        .iter()
        .map(|start| {
            let start_node = Node {
                xy: *start,
                value: values[*start],
            };
            rate_trailhead(&start_node, &graph, as_rating)
        })
        .sum())
}

fn main() {
    // let path: &str = "data/day10.sample";
    let path: &str = "data/day10.input";
    println!("Part 1: {:}", problem(path, false).unwrap());
    println!("Part 2: {:}", problem(path, true).unwrap());
}
