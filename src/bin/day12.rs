use advent_2024::{Graph, Grid};
use itertools::iproduct;

fn grid_from_str(input: &str) -> Grid<char> {
    let lines: Vec<&str> = input.trim().lines().collect();

    if lines.is_empty() {
        return Grid::new(0, 0, '.');
    }

    let height = lines.len();
    let width = lines[0].len();

    let data = lines.into_iter().flat_map(|line| line.chars()).collect();

    Grid {
        data,
        width,
        height,
    }
}

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
struct Plant {
    xy: (usize, usize),
    plant_type: char,
}

fn garden_as_graph(plants: Grid<char>) -> Graph<Plant> {
    iproduct!(0..plants.height, 0..plants.width)
        .map(|(x, y)| ((x, y), plants.cardinal_neighbors(x, y)))
        .fold(Graph::undirected(), |mut graph, (node, edges)| {
            let from = Plant {
                xy: node,
                plant_type: plants[node],
            };

            graph.add_node(from);

            for edge in edges {
                let plant_type = plants[edge];
                if plants[node] == plant_type {
                    let to = Plant {
                        xy: edge,
                        plant_type: plant_type,
                    };
                    graph.add_edge(from, to);
                }
            }
            graph
        })
}

fn calc_perimeter1(g: &Graph<Plant>) -> usize {
    g.nodes()
        .into_iter()
        .map(|node| 4 - g.neighbors(node).unwrap().iter().count())
        .sum()
}

fn graph_to_grid(graph: &Graph<Plant>) -> Grid<char> {
    // Get the extents of the plants in this subgraph
    let min_x = graph.nodes().iter().fold(usize::MAX, |a, b| a.min(b.xy.0));
    let min_y = graph.nodes().iter().fold(usize::MAX, |a, b| a.min(b.xy.1));
    let max_x = graph.nodes().iter().fold(0, |a, b| a.max(b.xy.0));
    let max_y = graph.nodes().iter().fold(0, |a, b| a.max(b.xy.1));

    // Add padding: +1 for 0-based index, +2 for padding
    let width = max_x - min_x + 3;
    let height = max_y - min_y + 3;

    // Construct a local grid for this subgraph
    let mut grid: Grid<char> = Grid::new(width, height, '.');
    for plant in graph.nodes() {
        let new_x = plant.xy.0 - min_x + 1;
        let new_y = plant.xy.1 - min_y + 1;
        grid[(new_x, new_y)] = plant.plant_type;
    }
    grid
}

fn calc_perimeter2(graph: &Graph<Plant>) -> usize {
    let grid: Grid<char> = graph_to_grid(graph);
    let mut n: usize = 0;
    // println!("\nProcessing grid ({} x {}):\n{}", grid.width, grid.height, grid);

    // Count horizontal edges
    for row in 0..grid.height - 1 {
        let mut is_upper_edge = false;
        let mut is_lower_edge = false;
        for col in 0..grid.width {
            let above = grid[(col, row)] != '.';
            let below = grid[(col, row + 1)] != '.';

            // Track upper and lower edges independently
            let is_upper_edge_now = above && !below;
            let is_lower_edge_now = !above && below;

            if (!is_upper_edge && is_upper_edge_now) || (!is_lower_edge && is_lower_edge_now) {
                // println!("Found horizontal edge at ({},{})", row, col);
                n += 1;
            }
            is_upper_edge = is_upper_edge_now;
            is_lower_edge = is_lower_edge_now;
        }
    }

    for col in 0..grid.width - 1 {
        let mut is_left_edge = false;
        let mut is_right_edge = false;
        for row in 0..grid.height {
            let left = grid[(col, row)] != '.';
            let right = grid[(col + 1, row)] != '.';
            let is_left_edge_now = left && !right;
            let is_right_edge_now = !left && right;
            if (!is_left_edge && is_left_edge_now) || (!is_right_edge && is_right_edge_now) {
                // println!("Found vertical edge at ({},{})", col, row);
                n += 1;
            }
            is_left_edge = is_left_edge_now;
            is_right_edge = is_right_edge_now;
        }
    }

    // println!("Has perimeter {n}\n");
    n
}

fn problem(input: &str, calc_perimeter: fn(&Graph<Plant>) -> usize) -> usize {
    // Find connected components, calculate cost for each, and add them up
    garden_as_graph(grid_from_str(input))
        .connected_components()
        .unwrap()
        .into_iter()
        .map(|g| {
            let area = g.nodes().len();
            let perimeter = calc_perimeter(&g);
            area * perimeter
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_pattern1() {
        let input = ["AAAA", "BBCD", "BBCC", "EEEC"].join("\n");
        assert_eq!(problem(&input, calc_perimeter1), 140);
    }

    #[test]
    fn test_part1_pattern2() {
        let input = ["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"].join("\n");
        assert_eq!(problem(&input, calc_perimeter1), 772);
    }

    #[test]
    fn test_part1_pattern3() {
        let input = [
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ]
        .join("\n");
        assert_eq!(problem(&input, calc_perimeter1), 1930);
    }

    #[test]
    fn test_part2_pattern1() {
        let input = ["AAAA", "BBCD", "BBCC", "EEEC"].join("\n");
        assert_eq!(problem(&input, calc_perimeter2), 80);
    }

    #[test]
    fn test_part2_pattern2() {
        let input = ["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"].join("\n");
        assert_eq!(problem(&input, calc_perimeter2), 436);
    }

    #[test]
    fn test_part2_pattern3() {
        let input = ["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"].join("\n");
        assert_eq!(problem(&input, calc_perimeter2), 236);
    }

    #[test]
    fn test_part2_pattern4() {
        let input = ["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"].join("\n");
        assert_eq!(problem(&input, calc_perimeter2), 368);
    }

    #[test]
    fn test_part2_pattern5() {
        let input = [
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ]
        .join("\n");
        assert_eq!(problem(&input, calc_perimeter2), 1206);
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("data/day12.input")?;
    println!("Part 1: {}", problem(&input, calc_perimeter1));
    println!("Part 2: {}", problem(&input, calc_perimeter2));
    Ok(())
}
