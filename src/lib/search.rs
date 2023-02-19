use crate::graph::{Graph, GraphType};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct BFS {}

#[derive(Debug)]
pub struct Dijkstra {}

pub trait SearchAlgorithm<T, G>
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
    fn shortest_path_util(g: &Graph<T, G>, source: T, target: T) -> Option<(usize, HashMap<T, T>)>;
    fn shortest_path(g: &Graph<T, G>, source: T, target: T) -> Option<Vec<T>> {
        Self::shortest_path_util(g, source.clone(), target.clone())
            .map(|(_, mut previous)| build_path::<T>(&mut previous, source, target))
    }
    fn shortest_path_length(g: &Graph<T, G>, source: T, target: T) -> Option<usize> {
        Self::shortest_path_util(g, source, target).map(|(len, _)| len)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T>
where
    T: Clone + Hash + Eq + Debug,
{
    cost: usize,
    position: T,
}

impl<T> Ord for State<T>
where
    T: Clone + Hash + Eq + Debug + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
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
            position: source.clone(),
        }]);
        *dist.get_mut(&source).unwrap() = 0;

        while let Some(State { cost, position }) = heap.pop() {
            if position == target {
                return Some((cost, previous));
            }
            if cost > dist[&position] {
                continue;
            }
            for neighbor in g.adj(&position).expect("No such node in a graph") {
                let next = State {
                    cost: cost + 1,
                    position: neighbor.clone(),
                };
                if next.cost < dist[&neighbor] {
                    *dist.get_mut(neighbor).unwrap() = next.cost;
                    heap.push(next);
                    previous.insert(neighbor.clone(), position.clone());
                }
            }
        }
        None
    }
}
