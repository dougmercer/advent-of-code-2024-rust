use advent_2024::Grid;
use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::{error::Error, fs};

const SUMMIT_HEIGHT: u8 = 9;
const TRAILHEAD_HEIGHT: u8 = 0;

fn parse_input(input: &str) -> Grid<u8> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Grid::new(0, 0, 0);
    }

    let height = lines.len();
    let width = lines[0].len();

    let data = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    Grid {
        data,
        width,
        height,
    }
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

fn problem(input: &str, as_rating: bool) -> usize {
    let values = parse_input(input);
    let graph = to_graph(&values);

    // Get list of trailheads (0s)
    let zeros: Vec<(usize, usize)> = iproduct!(0..values.height, 0..values.width)
        .filter(|(x, y)| values[(*x, *y)] == TRAILHEAD_HEIGHT)
        .collect();

    // Evaluate each trailhead
    zeros
        .iter()
        .map(|start| {
            let start_node = Node {
                xy: *start,
                value: values[*start],
            };
            rate_trailhead(&start_node, &graph, as_rating)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "data/day10.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:}", problem(&input, false));
    println!("Part 2: {:}", problem(&input, true));
    Ok(())
}

#[test]
fn test_part1() {
    let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;
    assert_eq!(problem(&input, false), 36);
}

#[test]
fn test_part2() {
    let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;
    assert_eq!(problem(&input, true), 81);
}
