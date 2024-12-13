use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Graph<N>
where
    N: Eq + Hash + Ord,
{
    pub edges: HashMap<N, HashSet<N>>,
}

impl<N> Graph<N>
where
    N: Eq + Hash + Clone + PartialOrd + Ord,
{
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: N) {
        self.edges.entry(node).or_default();
    }

    pub fn add_edge(&mut self, from: N, to: N) {
        self.edges
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
        self.edges.entry(to).or_default().insert(from);
    }

    pub fn add_directed_edge(&mut self, from: N, to: N) {
        self.edges.entry(from).or_default().insert(to);
    }

    pub fn is_undirected(&self) -> bool {
        self.edges.iter().all(|(from, to_set)| {
            to_set.iter().all(|to| {
                // Check if the reverse edge exists
                self.edges
                    .get(to)
                    .map(|neighbors| neighbors.contains(from))
                    .unwrap_or(false)
            })
        })
    }

    pub fn nodes(&self) -> Vec<&N> {
        self.edges
            .keys()
            .chain(self.edges.values().flatten())
            .unique()
            .sorted()
            .collect()
    }

    pub fn neighbors(&self, node: &N) -> Option<&HashSet<N>> {
        self.edges.get(node)
    }

    pub fn bfs(&self, start: N) -> Bfs<'_, N> {
        Bfs::new(self, start)
    }

    pub fn subgraph(&self, nodes: &[N]) -> Graph<N> {
        let mut subgraph: Graph<N> = Graph::new();

        for from in nodes {
            subgraph.add_node(from.clone());
            if let Some(neighbors) = self.neighbors(from) {
                for to in neighbors.iter().filter(|n| nodes.contains(n)) {
                    subgraph.add_edge(from.clone(), to.clone());
                }
            }
        }
        subgraph
    }

    pub fn connected_components(
        &self,
    ) -> Result<impl Iterator<Item = Graph<N>> + '_, &'static str> {
        if !self.is_undirected() {
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
}

pub trait GraphTraversal<N> {
    fn next_node(&mut self) -> Option<N>;
}

pub struct Bfs<'a, N>
where
    N: Eq + Hash + Clone + Ord,
{
    graph: &'a Graph<N>,
    queue: VecDeque<N>,
    visited: HashSet<N>,
}

impl<'a, N: Eq + Hash + Clone + Ord> Bfs<'a, N> {
    fn new(graph: &'a Graph<N>, start: N) -> Self {
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

impl<'a, N: Eq + Hash + Clone + Ord> Iterator for Bfs<'a, N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop_front()?;
        self.visited.insert(current.clone());
        if let Some(neighbors) = self.graph.neighbors(&current) {
            for neighbor in neighbors {
                if !self.visited.contains(neighbor) {
                    self.visited.insert(neighbor.clone());
                    self.queue.push_back(neighbor.clone());
                }
            }
        }
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph: Graph<i32> = Graph::new();
        assert!(graph.neighbors(&1).is_none());
    }

    #[test]
    fn test_single_edge() {
        let mut graph = Graph::new();
        graph.add_directed_edge(1, 2);

        let neighbors = graph.neighbors(&1).unwrap();
        assert_eq!(neighbors.len(), 1);
        assert!(neighbors.contains(&2));
        assert!(graph.neighbors(&2).is_none());
    }

    #[test]
    fn test_multiple_edges() {
        let mut graph = Graph::new();
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
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let path: Vec<i32> = graph.bfs(1).collect();
        assert_eq!(path, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_bfs_with_branches() {
        let mut graph = Graph::new();
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
        let mut graph = Graph::new();
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 3);
        graph.add_directed_edge(3, 1);

        let path: Vec<i32> = graph.bfs(1).collect();
        assert_eq!(path, vec![1, 2, 3]);
    }

    #[test]
    fn test_bfs_disconnected() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        let path: Vec<i32> = graph.bfs(1).collect();
        assert_eq!(path, vec![1, 2]);
    }

    #[test]
    fn test_string_nodes() {
        let mut graph = Graph::new();
        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "c".to_string());

        let path: Vec<String> = graph.bfs("a".to_string()).collect();
        assert_eq!(path, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_nodes() {
        let mut graph = Graph::new();
        graph.add_edge("a".to_string(), "b".to_string());
        graph.add_edge("b".to_string(), "c".to_string());

        let nodes: Vec<&String> = graph.nodes();
        assert_eq!(nodes, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_custom_type() {
        #[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
        struct CustomNode {
            id: i32,
            letter: char,
        }

        let mut graph = Graph::new();
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
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let sub = graph.subgraph(&[1, 2]);
        let path: Vec<i32> = sub.bfs(1).collect();
        assert_eq!(path, vec![1, 2]);
        let nodes = sub.nodes();
        assert_eq!(nodes, vec![&1, &2]);
    }

    #[test]
    fn test_connected_components() {
        let mut graph = Graph::new();

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
}
