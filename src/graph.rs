use itertools::Itertools;
use num_traits::{Bounded, NumOps, One, Zero};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign};

pub trait Weight:
    Clone + Copy + PartialOrd + Debug + Display + Zero + One + Bounded + NumOps + Add + AddAssign
{
}
impl<T> Weight for T where
    T: Clone
        + Copy
        + PartialOrd
        + Debug
        + Display
        + Zero
        + One
        + Bounded
        + NumOps
        + Add
        + AddAssign
{
}

#[derive(Debug, Default)]
pub struct Graph<N, W = u32>
where
    N: Eq + Hash + Ord,
    W: Weight,
{
    adjacency_map: HashMap<N, HashMap<N, W>>,
    directed: bool,
}

impl<N, W> Graph<N, W>
where
    N: Eq + Hash + Clone + Ord,
    W: Weight + Clone + Default,
{
    pub fn new(directed: bool) -> Self {
        Self {
            adjacency_map: HashMap::new(),
            directed,
        }
    }

    pub fn undirected() -> Self {
        Self::new(false)
    }

    pub fn directed() -> Self {
        Self::new(true)
    }

    pub fn add_node(&mut self, node: N) {
        self.adjacency_map.entry(node).or_default();
    }

    pub fn add_edge(&mut self, from: N, to: N) {
        self.add_edge_weighted(from, to, W::one())
    }

    pub fn add_edge_weighted(&mut self, from: N, to: N, weight: W) {
        if self.directed {
            self.adjacency_map
                .entry(from.clone())
                .or_default()
                .insert(to, weight);
        } else {
            self.adjacency_map
                .entry(from.clone())
                .or_default()
                .insert(to.clone(), weight.clone());
            self.adjacency_map
                .entry(to)
                .or_default()
                .insert(from, weight);
        }
    }

    pub fn nodes(&self) -> Vec<&N> {
        self.adjacency_map
            .keys()
            .chain(self.adjacency_map.values().flat_map(|m| m.keys()))
            .unique()
            .sorted()
            .collect()
    }

    pub fn edges(&self) -> impl Iterator<Item = (&N, &N, &W)> {
        self.adjacency_map
            .iter()
            .flat_map(|(from, edges)| edges.iter().map(move |(to, weight)| (from, to, weight)))
    }

    pub fn edge_pairs(&self) -> impl Iterator<Item = (&N, &N)> {
        self.adjacency_map
            .iter()
            .flat_map(|(from, edges)| edges.keys().map(move |to| (from, to)))
    }

    pub fn has_edge(&self, from: &N, to: &N) -> bool {
        self.adjacency_map
            .get(from)
            .map_or(false, |edges| edges.contains_key(to))
    }

    pub fn get_weight(&self, from: &N, to: &N) -> Option<&W> {
        self.adjacency_map.get(from)?.get(to)
    }

    pub fn neighbors(&self, node: &N) -> Option<HashSet<N>> {
        self.adjacency_map
            .get(node)
            .map(|neighbors| neighbors.keys().cloned().collect())
    }

    pub fn neighbors_weighted(&self, node: &N) -> Option<&HashMap<N, W>> {
        self.adjacency_map.get(node)
    }

    pub fn bfs(&self, start: N) -> Bfs<'_, N, W> {
        Bfs::new(self, start)
    }

    pub fn subgraph(&self, nodes: &[N]) -> Graph<N, W> {
        let mut subgraph: Graph<N, W> = Graph::new(self.directed);

        for from in nodes {
            subgraph.add_node(from.clone());
            if let Some(neighbors) = self.neighbors_weighted(from) {
                for (to, weight) in neighbors {
                    if nodes.contains(to) {
                        subgraph.add_edge_weighted(from.clone(), to.clone(), weight.clone());
                    }
                }
            }
        }
        subgraph
    }

    pub fn connected_components(
        &self,
    ) -> Result<impl Iterator<Item = Graph<N, W>> + '_, &'static str> {
        if self.directed {
            return Err("Cannot find connected components of a directed graph");
        }

        // Note: Iterate in rev order so that we pop from front of nodes
        let mut nodes: Vec<N> = self.nodes().into_iter().cloned().rev().collect();
        let mut visited = HashSet::new();

        Ok(std::iter::from_fn(move || {
            while let Some(node) = nodes.pop() {
                if !visited.contains(&node) {
                    let component: Vec<_> = self.bfs(node).collect();
                    // Remove nodes from this component
                    nodes.retain(|n| !component.contains(n));
                    // Add them to visited set
                    visited.extend(component.iter().cloned());
                    return Some(self.subgraph(&component));
                }
            }
            None
        }))
    }

    pub fn shortest_path(&self, start: N, end: N) -> Option<(Vec<N>, W)>
    where
        N: Eq + Hash + Clone + Ord,
        W: Weight + Clone + Default + Eq,
    {
        let mut dijkstra = Dijkstra::new(self, start);
        dijkstra.shortest_path(&end)
    }
}

pub trait GraphTraversal<N> {
    fn next_node(&mut self) -> Option<N>;
}

pub struct Bfs<'a, N, W>
where
    N: Eq + Hash + Clone + Ord,
    W: Weight,
{
    graph: &'a Graph<N, W>,
    queue: VecDeque<N>,
    visited: HashSet<N>,
}

impl<'a, N, W> Bfs<'a, N, W>
where
    N: Eq + Hash + Clone + Ord,
    W: Weight + Clone + Default,
{
    fn new(graph: &'a Graph<N, W>, start: N) -> Self {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start.clone());
        queue.push_back(start);

        Self {
            graph,
            queue,
            visited,
        }
    }
}

impl<'a, N, W> Iterator for Bfs<'a, N, W>
where
    N: Eq + Hash + Clone + Ord,
    W: Weight + Clone + Default,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop_front()?;
        self.visited.insert(current.clone());
        if let Some(neighbors) = self.graph.neighbors(&current) {
            for neighbor in neighbors {
                if !self.visited.contains(&neighbor) {
                    self.visited.insert(neighbor.clone());
                    self.queue.push_back(neighbor.clone());
                }
            }
        }
        Some(current)
    }
}

pub struct Dijkstra<'a, N, W>
where
    N: Eq + Hash + Clone + Ord,
    W: Weight + Clone + Default,
{
    graph: &'a Graph<N, W>,
    distances: HashMap<N, W>,
    predecessors: HashMap<N, N>,
    queue: BinaryHeap<State<N, W>>,
}

// Update State to use the weight type directly
#[derive(Eq, PartialEq)]
struct State<N, W> {
    node: N,
    distance: W,
}

// Update Ord implementation to use PartialOrd from Weight trait
impl<N: Ord, W: PartialOrd + Eq> Ord for State<N, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip ordering for min-heap
        match other.distance.partial_cmp(&self.distance) {
            Some(o) => o.then_with(|| self.node.cmp(&other.node)),
            None => self.node.cmp(&other.node),
        }
    }
}

impl<N: Ord, W: PartialOrd + Eq> PartialOrd for State<N, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, N, W> Dijkstra<'a, N, W>
where
    N: Eq + Hash + Clone + Ord,
    W: Weight + Clone + Default + Eq,
{
    pub fn new(graph: &'a Graph<N, W>, start: N) -> Self {
        let mut dijkstra = Self {
            graph,
            distances: HashMap::new(),
            predecessors: HashMap::new(),
            queue: BinaryHeap::new(),
        };

        // Initialize start node
        dijkstra.distances.insert(start.clone(), W::zero());
        dijkstra.queue.push(State {
            node: start,
            distance: W::zero(),
        });

        dijkstra
    }

    pub fn shortest_path(&mut self, end: &N) -> Option<(Vec<N>, W)> {
        while let Some(State { node, distance }) = self.queue.pop() {
            if &node == end {
                return Some((self.reconstruct_path(end), distance));
            }

            if let Some(best) = self.distances.get(&node) {
                if distance > *best {
                    continue;
                }
            }

            if let Some(neighbors) = self.graph.neighbors_weighted(&node) {
                for (next, weight) in neighbors {
                    let mut next_distance = distance.clone();
                    next_distance += weight.clone();

                    if !self.distances.contains_key(next) || next_distance < self.distances[next] {
                        self.distances.insert(next.clone(), next_distance.clone());
                        self.predecessors.insert(next.clone(), node.clone());
                        self.queue.push(State {
                            node: next.clone(),
                            distance: next_distance,
                        });
                    }
                }
            }
        }
        None
    }

    fn reconstruct_path(&self, end: &N) -> Vec<N> {
        let mut path = vec![end.clone()];
        let mut current = end;

        while let Some(predecessor) = self.predecessors.get(current) {
            path.push(predecessor.clone());
            current = predecessor;
        }

        path.reverse();
        path
    }
}

impl<'a, N, W> Dijkstra<'a, N, W>
where
    N: Eq + Hash + Clone + Ord,
    W: Weight + Clone + Default + Eq,
{
    pub fn all_shortest_paths(&mut self, end: &N) -> Option<(Vec<Vec<N>>, W)> {
        // Track all predecessors for each node
        let mut all_predecessors: HashMap<N, Vec<N>> = HashMap::new();

        while let Some(State { node, distance }) = self.queue.pop() {
            if &node == end {
                return Some((self.reconstruct_all_paths(end, &all_predecessors), distance));
            }

            if let Some(best) = self.distances.get(&node) {
                if distance > *best {
                    continue;
                }
            }

            if let Some(neighbors) = self.graph.neighbors_weighted(&node) {
                for (next, weight) in neighbors {
                    let mut next_distance = distance.clone();
                    next_distance += weight.clone();

                    match self.distances.get(next) {
                        Some(current_best) if next_distance > *current_best => continue,
                        Some(current_best) if next_distance == *current_best => {
                            // Found another path with same distance
                            all_predecessors
                                .entry(next.clone())
                                .or_default()
                                .push(node.clone());
                        }
                        _ => {
                            // Found better path
                            self.distances.insert(next.clone(), next_distance.clone());
                            all_predecessors.entry(next.clone()).or_default().clear();
                            all_predecessors
                                .entry(next.clone())
                                .or_default()
                                .push(node.clone());
                            self.queue.push(State {
                                node: next.clone(),
                                distance: next_distance,
                            });
                        }
                    }
                }
            }
        }
        None
    }

    fn reconstruct_all_paths(&self, end: &N, all_predecessors: &HashMap<N, Vec<N>>) -> Vec<Vec<N>> {
        let mut all_paths = Vec::new();
        let mut current_path = Vec::new();

        self.build_paths(end, all_predecessors, &mut current_path, &mut all_paths);

        // Reverse each path since we built them backwards
        all_paths.iter_mut().for_each(|path| path.reverse());
        all_paths
    }

    fn build_paths(
        &self,
        current: &N,
        all_predecessors: &HashMap<N, Vec<N>>,
        current_path: &mut Vec<N>,
        all_paths: &mut Vec<Vec<N>>,
    ) {
        current_path.push(current.clone());

        if let Some(predecessors) = all_predecessors.get(current) {
            for predecessor in predecessors {
                self.build_paths(predecessor, all_predecessors, current_path, all_paths);
            }
        } else {
            // Reached the start node
            all_paths.push(current_path.clone());
        }

        current_path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph: Graph<i32> = Graph::undirected();
        assert!(graph.neighbors(&1).is_none());
    }

    #[test]
    fn test_single_edge() {
        let mut graph: Graph<i32> = Graph::directed();
        graph.add_edge(1, 2);

        let neighbors = graph.neighbors(&1).unwrap();
        assert_eq!(neighbors.len(), 1);
        assert!(neighbors.contains(&2));
        assert!(graph.neighbors(&2).is_none());
    }

    #[test]
    fn test_multiple_edges() {
        let mut graph: Graph<i32> = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);

        let neighbors = graph.neighbors(&1).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&2));
        assert!(neighbors.contains(&3));
    }

    #[test]
    fn test_bfs_simple_path() {
        let mut graph: Graph<i32> = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let path: Vec<i32> = graph.bfs(1).collect();
        assert_eq!(path, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_bfs_with_branches() {
        let mut graph: Graph<i32> = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 5);

        let path: Vec<i32> = graph.bfs(1).collect();

        assert_eq!(path[0], 1);
        assert!(path[1..3].contains(&2));
        assert!(path[1..3].contains(&3));
        assert!(path[3..5].contains(&4));
        assert!(path[3..5].contains(&5));
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph: Graph<i32> = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);

        let path: Vec<i32> = graph.bfs(1).collect();
        assert_eq!(path, vec![1, 2, 3]);
    }

    #[test]
    fn test_bfs_disconnected() {
        let mut graph: Graph<i32> = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        let path: Vec<i32> = graph.bfs(1).collect();
        assert_eq!(path, vec![1, 2]);
    }

    #[test]
    fn test_string_bfs() {
        let mut graph: Graph<char> = Graph::directed();
        graph.add_edge('a', 'b');
        graph.add_edge('b', 'c');

        let path: Vec<char> = graph.bfs('a').collect();
        assert_eq!(path, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_nodes() {
        let mut graph: Graph<char> = Graph::directed();
        graph.add_edge('a', 'b');
        graph.add_edge('b', 'c');

        let nodes: Vec<&char> = graph.nodes();
        assert_eq!(nodes, vec![&'a', &'b', &'c']);
    }

    #[test]
    fn test_custom_type() {
        #[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
        struct CustomNode {
            id: i32,
            letter: char,
        }

        let mut graph: Graph<CustomNode> = Graph::directed();
        let node1 = CustomNode { id: 1, letter: 'a' };
        let node2 = CustomNode { id: 2, letter: 'b' };

        graph.add_edge(node1.clone(), node2.clone());

        let path: Vec<CustomNode> = graph.bfs(node1).collect();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].id, 1);
        assert_eq!(path[1].id, 2);
    }

    #[test]
    fn test_subgraph() {
        let mut graph: Graph<i32> = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let sub = graph.subgraph(&[1, 2]);
        let path: Vec<i32> = sub.bfs(1).collect();
        assert_eq!(path, vec![1, 2]);
    }

    #[test]
    fn test_connected_components() {
        let mut graph = Graph::undirected();

        // Create two separate components:
        // Component 1: 1-2-3
        // Component 2: 4-5
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(4, 5);

        let components: Vec<Graph<i32>> = graph.connected_components().unwrap().collect();

        println!("{:?}", components);
        assert_eq!(components.len(), 2);

        let component1_nodes: Vec<_> = components[0].nodes().into_iter().cloned().collect();
        let expected1 = vec![1, 2, 3];
        assert_eq!(component1_nodes, expected1);

        let component2_nodes: Vec<_> = components[1].nodes().into_iter().cloned().collect();
        let expected2 = vec![4, 5];
        assert_eq!(component2_nodes, expected2);

        assert!(components[0].neighbors(&1).unwrap().contains(&2));
        assert!(components[0].neighbors(&2).unwrap().contains(&3));
        assert!(components[1].neighbors(&4).unwrap().contains(&5));
    }

    #[test]
    fn test_bfs_weighted_graph() {
        let mut graph: Graph<i32, f64> = Graph::directed();

        // Create a weighted graph:
        graph.add_edge_weighted(1, 2, 2.5);
        graph.add_edge_weighted(1, 3, 1.0);
        graph.add_edge_weighted(2, 4, 1.0);
        graph.add_edge_weighted(3, 5, 0.5);
        graph.add_edge_weighted(4, 5, 0.8);

        // BFS from node 1 should visit nodes in order of their distance
        // (in terms of number of edges, not weights)
        let path: Vec<i32> = graph.bfs(1).collect();

        // First level: immediate neighbors of 1
        assert_eq!(path[0], 1);
        assert!(path[1..3].contains(&2));
        assert!(path[1..3].contains(&3));

        // Second level: nodes two edges away from 1
        assert!(path[3..5].contains(&4));
        assert!(path[3..5].contains(&5));

        // Also verify we get all nodes exactly once
        assert_eq!(path.len(), 5);
        let unique: HashSet<_> = path.iter().collect();
        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn test_weighted_subgraph() {
        let mut graph: Graph<i32, f64> = Graph::directed();
        graph.add_edge_weighted(1, 2, 2.5);
        graph.add_edge_weighted(2, 3, 1.0);
        graph.add_edge_weighted(3, 4, 0.5);

        let sub = graph.subgraph(&[1, 2, 3]);

        // Check nodes
        assert_eq!(sub.nodes(), vec![&1, &2, &3]);

        // Check weights are preserved
        assert_eq!(sub.get_weight(&1, &2), Some(&2.5));
        assert_eq!(sub.get_weight(&2, &3), Some(&1.0));
        assert_eq!(sub.get_weight(&1, &3), None);
    }

    #[test]
    fn test_weighted_connected_components() {
        let mut graph: Graph<i32, f64> = Graph::undirected();

        // Component 1
        graph.add_edge_weighted(1, 2, 1.5);
        graph.add_edge_weighted(2, 3, 2.0);
        graph.add_edge_weighted(3, 1, 2.5);

        // Component 2
        graph.add_edge_weighted(4, 5, 3.0);

        let components: Vec<Graph<i32, f64>> = graph.connected_components().unwrap().collect();

        assert_eq!(components.len(), 2);

        // Check first component
        let c1 = &components[0];
        assert_eq!(c1.nodes(), vec![&1, &2, &3]);
        assert_eq!(c1.get_weight(&1, &2), Some(&1.5));
        assert_eq!(c1.get_weight(&2, &3), Some(&2.0));
        assert_eq!(c1.get_weight(&3, &1), Some(&2.5));

        // Check second component
        let c2 = &components[1];
        assert_eq!(c2.nodes(), vec![&4, &5]);
        assert_eq!(c2.get_weight(&4, &5), Some(&3.0));
    }

    #[test]
    fn test_directed_weighted_edges() {
        let mut graph: Graph<i32, i32> = Graph::directed();

        graph.add_edge_weighted(1, 2, 10);
        assert_eq!(graph.get_weight(&1, &2), Some(&10));
        assert_eq!(graph.get_weight(&2, &1), None);
    }

    #[test]
    fn test_modify_weights() {
        let mut graph: Graph<i32, i32> = Graph::undirected();

        // Add edge with initial weight
        graph.add_edge_weighted(1, 2, 10);
        assert_eq!(graph.get_weight(&1, &2), Some(&10));

        // Modify weight
        graph.add_edge_weighted(1, 2, 20);
        assert_eq!(graph.get_weight(&1, &2), Some(&20));
        assert_eq!(graph.get_weight(&2, &1), Some(&20)); // Check both directions
    }

    #[test]
    fn test_empty_weighted_graph() {
        let graph: Graph<i32, f64> = Graph::directed();
        assert!(graph.nodes().is_empty());
        assert_eq!(graph.get_weight(&1, &2), None);
    }
}

#[cfg(test)]
mod dijkstra_tests {
    use super::*;

    #[test]
    fn test_dijkstra_simple_path() {
        let mut graph: Graph<i32, usize> = Graph::directed();
        graph.add_edge_weighted(1, 2, 4);
        graph.add_edge_weighted(2, 3, 3);
        graph.add_edge_weighted(3, 4, 5);
        graph.add_edge_weighted(1, 4, 15); // Longer direct path

        let mut dijkstra = Dijkstra::new(&graph, 1);
        let (path, distance) = dijkstra.shortest_path(&4).unwrap();

        assert_eq!(path, vec![1, 2, 3, 4]);
        assert_eq!(distance, 12); // 4 + 3 + 5 = 12
    }

    #[test]
    fn test_dijkstra_no_path() {
        let mut graph: Graph<i32, usize> = Graph::directed();
        // Two components
        graph.add_edge_weighted(1, 2, 1);
        graph.add_edge_weighted(3, 4, 1);

        let mut dijkstra = Dijkstra::new(&graph, 1);
        assert_eq!(dijkstra.shortest_path(&4), None);
    }

    #[test]
    fn test_dijkstra_zero_weight() {
        let mut graph: Graph<i32, usize> = Graph::directed();
        graph.add_edge_weighted(1, 2, 0);
        graph.add_edge_weighted(2, 3, 0);
        graph.add_edge_weighted(3, 4, 1);

        let mut dijkstra = Dijkstra::new(&graph, 1);
        let (path, distance) = dijkstra.shortest_path(&4).unwrap();

        assert_eq!(path, vec![1, 2, 3, 4]);
        assert_eq!(distance, 1);
    }

    #[test]
    fn test_dijkstra_complex_graph() {
        let mut graph: Graph<i32, usize> = Graph::directed();
        // Create a more complex graph:
        //    1 --4-- 2 --3-- 3
        //    |       |       |
        //    8       2       5
        //    |       |       |
        //    4 --3-- 5 --2-- 6

        let edges = vec![
            (1, 2, 4),
            (2, 3, 3),
            (1, 4, 8),
            (2, 5, 2),
            (3, 6, 5),
            (4, 5, 3),
            (5, 6, 2),
        ];

        for (from, to, weight) in edges {
            graph.add_edge_weighted(from, to, weight);
        }

        let mut dijkstra = Dijkstra::new(&graph, 1);
        let (path, distance) = dijkstra.shortest_path(&6).unwrap();

        // Shortest path should be 1 -> 2 -> 5 -> 6 (total: 8)
        assert_eq!(path, vec![1, 2, 5, 6]);
        assert_eq!(distance, 8); // 4 + 2 + 2 = 8
    }

    #[test]
    fn test_dijkstra_unweighted() {
        let mut graph: Graph<i32> = Graph::directed();
        // All edges weight 1
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(1, 4);
        graph.add_edge(4, 3);

        let mut dijkstra = Dijkstra::new(&graph, 1);
        let (path, distance) = dijkstra.shortest_path(&3).unwrap();

        // Both paths (1->2->3 and 1->4->3) are equal length
        assert_eq!(distance, 2);
        assert!(path == vec![1, 2, 3] || path == vec![1, 4, 3]);
    }

    #[test]
    fn test_dijkstra_self_loop() {
        let mut graph: Graph<i32, usize> = Graph::directed();
        graph.add_edge_weighted(1, 1, 1); // Self loop
        graph.add_edge_weighted(1, 2, 2);

        let mut dijkstra = Dijkstra::new(&graph, 1);
        let (path, distance) = dijkstra.shortest_path(&2).unwrap();

        assert_eq!(path, vec![1, 2]);
        assert_eq!(distance, 2); // Should ignore self loop
    }

    #[test]
    fn test_dijkstra_custom_type() {
        #[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
        struct Node {
            id: i32,
            name: String,
        }

        let mut graph: Graph<Node, usize> = Graph::directed();
        let n1 = Node {
            id: 1,
            name: "A".to_string(),
        };
        let n2 = Node {
            id: 2,
            name: "B".to_string(),
        };
        let n3 = Node {
            id: 3,
            name: "C".to_string(),
        };

        graph.add_edge_weighted(n1.clone(), n2.clone(), 5);
        graph.add_edge_weighted(n2.clone(), n3.clone(), 3);

        let mut dijkstra = Dijkstra::new(&graph, n1.clone());
        let (path, distance) = dijkstra.shortest_path(&n3).unwrap();

        assert_eq!(path.len(), 3);
        assert_eq!(path[0].name, "A");
        assert_eq!(path[1].name, "B");
        assert_eq!(path[2].name, "C");
        assert_eq!(distance, 8);
    }
}

#[cfg(test)]
mod all_shortest_paths_tests {
    use super::*;

    #[test]
    fn test_multiple_shortest_paths() {
        let mut graph: Graph<i32, usize> = Graph::directed();

        // Create a graph with multiple equal-length paths from 1 to 4
        //     2
        //   /   \
        // 1       4
        //   \   /
        //     3
        graph.add_edge_weighted(1, 2, 1);
        graph.add_edge_weighted(1, 3, 1);
        graph.add_edge_weighted(2, 4, 1);
        graph.add_edge_weighted(3, 4, 1);

        let mut dijkstra = Dijkstra::new(&graph, 1);
        let (paths, distance) = dijkstra.all_shortest_paths(&4).unwrap();

        assert_eq!(distance, 2);
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&vec![1, 2, 4]));
        assert!(paths.contains(&vec![1, 3, 4]));
    }

    #[test]
    fn test_single_shortest_path() {
        let mut graph: Graph<i32, usize> = Graph::directed();
        graph.add_edge_weighted(1, 2, 1);
        graph.add_edge_weighted(2, 3, 1);
        graph.add_edge_weighted(1, 3, 3); // Longer alternative path

        let mut dijkstra = Dijkstra::new(&graph, 1);
        let (paths, distance) = dijkstra.all_shortest_paths(&3).unwrap();

        assert_eq!(distance, 2);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![1, 2, 3]);
    }

    #[test]
    fn test_no_path() {
        let mut graph: Graph<i32, usize> = Graph::directed();
        graph.add_edge_weighted(1, 2, 1);
        graph.add_edge_weighted(3, 4, 1); // Disconnected

        let mut dijkstra = Dijkstra::new(&graph, 1);
        assert!(dijkstra.all_shortest_paths(&4).is_none());
    }
}
