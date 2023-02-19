//! Path searching in a graph.
use crate::graph::{Graph, GraphType};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

/// Breadth-first search (BFS) algorithm.
#[derive(Debug)]
pub struct BFS {}

/// Dijkstra's algorithm for a weighted graph.
#[derive(Debug)]
pub struct Dijkstra {}

pub trait SearchAlgorithm<T, G>
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
    /// A utility function for finding a shortest path in a graph.
    fn shortest_path_util(g: &Graph<T, G>, source: T, target: T) -> Option<(usize, HashMap<T, T>)>;

    /// A shortest path between a `source` and a `target` nodes in a graph `g`.
    fn shortest_path(g: &Graph<T, G>, source: T, target: T) -> Option<Vec<T>> {
        Self::shortest_path_util(g, source.clone(), target.clone())
            .map(|(_, mut previous)| build_path::<T>(&mut previous, source, target))
    }

    /// A shortest path's length.
    fn shortest_path_length(g: &Graph<T, G>, source: T, target: T) -> Option<usize> {
        Self::shortest_path_util(g, source, target).map(|(len, _)| len)
    }

    /// Returns `True` if `g` has a path from `source` to `target`
    fn has_path(g: &Graph<T, G>, source: T, target: T) -> bool {
        Self::shortest_path_util(g, source, target).is_some()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T>
where
    T: Clone + Hash + Eq + Debug,
{
    cost: usize,
    node: T,
}

impl<T> Ord for State<T>
where
    T: Clone + Hash + Eq + Debug + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for State<T>
where
    T: Clone + Hash + Eq + Debug + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_path<T>(previous: &mut HashMap<T, T>, source: T, target: T) -> Vec<T>
where
    T: Clone + Hash + Eq + Debug,
{
    let mut path: Vec<T> = vec![target.clone()];
    let mut current: T = target;

    while current != source {
        current = previous.remove(&current).expect("No entry");
        path.push(current.clone());
    }
    path.into_iter().rev().collect()
}

impl<T, G> SearchAlgorithm<T, G> for BFS
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
    fn shortest_path_util(g: &Graph<T, G>, source: T, target: T) -> Option<(usize, HashMap<T, T>)> {
        let mut previous: HashMap<T, T> = HashMap::new();
        let mut visited: HashSet<T> = HashSet::from_iter(vec![source.clone()]);
        let mut queue: VecDeque<(T, usize)> = VecDeque::from_iter(vec![(source.clone(), 0)]);

        while queue.len() > 0 {
            let node: T;
            let dist: usize;
            (node, dist) = queue.pop_front().expect("Empty queue");

            if node == target {
                return Some((dist, previous));
            }

            for neighbor in g.adj(&node).expect("Node does not exist") {
                if !visited.contains(neighbor) {
                    previous.insert(neighbor.clone(), node.clone());
                    queue.push_back((neighbor.clone(), dist + 1));
                    visited.insert(neighbor.clone());
                }
            }
        }
        None
    }
}

impl<T, G> SearchAlgorithm<T, G> for Dijkstra
where
    T: Clone + Hash + Eq + Debug + Ord,
    G: GraphType,
{
    fn shortest_path_util(g: &Graph<T, G>, source: T, target: T) -> Option<(usize, HashMap<T, T>)> {
        let mut dist: HashMap<T, usize> = g
            .nodes::<Vec<_>>()
            .iter()
            .map(|x| ((*x).clone(), usize::MAX))
            .collect();
        let mut previous: HashMap<T, T> = HashMap::new();
        let mut heap: BinaryHeap<State<T>> = BinaryHeap::from([State {
            cost: 0,
            node: source.clone(),
        }]);
        *dist.get_mut(&source).unwrap() = 0;

        while let Some(State { cost, node }) = heap.pop() {
            if node == target {
                return Some((cost, previous));
            }
            if cost > dist[&node] {
                continue;
            }
            for neighbor in g.adj(&node).expect("No such node in a graph") {
                let next = State {
                    cost: cost + 1,
                    node: neighbor.clone(),
                };
                if next.cost < dist[&neighbor] {
                    *dist.get_mut(neighbor).unwrap() = next.cost;
                    heap.push(next);
                    previous.insert(neighbor.clone(), node.clone());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_graph() -> Graph<i8> {
        let mut g: Graph<i8> = Graph::new();
        g.add_edges_from(vec![(1, 2), (2, 3), (3, 4), (1, 5), (5, 4), (4, 6)]);
        g.add_node(7);
        g
    }

    #[test]
    fn bfs_shortest_path_exists() {
        let g = simple_graph();
        let actual = BFS::shortest_path(&g, 1, 6);
        let expected = Some(vec![1, 5, 4, 6]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn bfs_shortest_path_not_exists() {
        let g = simple_graph();
        let actual = BFS::shortest_path(&g, 1, 7);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn bfs_shortest_path_len_exists() {
        let g = simple_graph();
        let actual = BFS::shortest_path_length(&g, 1, 6);
        let expected = Some(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn bfs_shortest_path_len_not_exists() {
        let g = simple_graph();
        let actual = BFS::shortest_path_length(&g, 1, 7);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn bfs_has_path() {
        let g = simple_graph();
        let actual = BFS::has_path(&g, 1, 6);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn bfs_has_not_path() {
        let g = simple_graph();
        let actual = BFS::has_path(&g, 1, 7);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn dijkstra_shortest_path_exists() {
        let g = simple_graph();
        let actual = Dijkstra::shortest_path(&g, 1, 6);
        let expected = Some(vec![1, 5, 4, 6]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dijkstra_shortest_path_not_exists() {
        let g = simple_graph();
        let actual = Dijkstra::shortest_path(&g, 1, 7);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn dijkstra_shortest_path_len_exists() {
        let g = simple_graph();
        let actual = Dijkstra::shortest_path_length(&g, 1, 6);
        let expected = Some(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dijkstra_shortest_path_len_not_exists() {
        let g = simple_graph();
        let actual = Dijkstra::shortest_path_length(&g, 1, 7);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn dijkstra_has_path() {
        let g = simple_graph();
        let actual = Dijkstra::has_path(&g, 1, 6);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn dijkstra_has_not_path() {
        let g = simple_graph();
        let actual = Dijkstra::has_path(&g, 1, 7);
        let expected = false;
        assert_eq!(actual, expected);
    }
}
