use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Undirected {}

#[derive(Debug)]
pub struct Directed {}

pub trait GraphType {}

impl GraphType for Undirected {}
impl GraphType for Directed {}

#[derive(Debug)]
pub struct Graph<T, G = Undirected>
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
    adj: HashMap<T, HashSet<T>>,
    pred: HashMap<T, HashSet<T>>,
    typ: PhantomData<G>,
}

pub type DiGraph<T> = Graph<T, Directed>;

impl<T, G> Default for Graph<T, G>
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, G> Graph<T, G>
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
    /// Create an empty graph.
    pub fn new() -> Self {
        Graph {
            adj: HashMap::new(),
            pred: HashMap::new(),
            typ: PhantomData,
        }
    }

    /// Iterate over a graph, i.e. over its keys.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.adj.keys()
    }

    /// Get all nodes from a graph.
    pub fn nodes<B: FromIterator<T>>(&self) -> B {
        self.adj.keys().cloned().collect()
    }

    pub fn edges<B: FromIterator<(T, T)>>(&self) -> B {
        self.adj
            .clone()
            .into_iter()
            .flat_map(|(k, v)| v.into_iter().map(move |w| (k.clone(), w)))
            .collect()
    }

    /// Adds a directed edge from u to v (u->v).
    fn add_directed_edge(&mut self, u: T, v: T) {
        self.adj.entry(u).or_default().insert(v);
    }

    /// Get adjacent elements in a graph.
    pub fn adj(&self, u: &T) -> Option<&HashSet<T>> {
        self.adj.get(u)
    }
}

impl<T> Graph<T, Undirected>
where
    T: Clone + Hash + Eq + Debug,
{
    pub fn is_directed() -> bool {
        false
    }

    /// Add a node. Do nothing if it already exists.
    pub fn add_node(&mut self, u: T) {
        self.adj.entry(u).or_default();
    }

    /// Adds an edge in a graph (u<->v).
    pub fn add_edge(&mut self, u: T, v: T) {
        self.add_directed_edge(u.clone(), v.clone());
        self.add_directed_edge(v, u);
    }

    /// Add many edges at once
    pub fn add_edges_from(&mut self, edges: Vec<(T, T)>) {
        for edge in edges.into_iter() {
            self.add_edge(edge.0, edge.1);
        }
    }
}

impl<T> Graph<T, Directed>
where
    T: Clone + Hash + Eq + Debug,
{
    pub fn is_directed() -> bool {
        true
    }

    /// Add a node. Do nothing if it already exists.
    pub fn add_node(&mut self, u: T) {
        self.adj.entry(u.clone()).or_default();
        self.pred.entry(u).or_default();
    }

    /// Adds an edge in a graph (u->v).
    pub fn add_edge(&mut self, u: T, v: T) {
        self.adj.entry(u.clone()).or_default().insert(v.clone());
        self.adj.entry(v.clone()).or_default();

        self.pred.entry(v).or_default().insert(u.clone());
        self.pred.entry(u).or_default();
    }

    /// Add many edges at once
    pub fn add_edges_from(&mut self, edges: Vec<(T, T)>) {
        for edge in edges.into_iter() {
            self.add_edge(edge.0, edge.1);
        }
    }

    pub fn in_degree(&self, u: &T) -> usize {
        match self.pred.get(u) {
            Some(v) => v.len(),
            _ => 0,
        }
    }

    pub fn in_degree_map(&self) -> HashMap<T, usize> {
        self.nodes::<Vec<_>>()
            .iter()
            .map(|n| (n.clone(), self.in_degree(n)))
            .collect::<HashMap<T, usize>>()
    }
}

pub fn topological_generations<T>(g: &Graph<T, Directed>) -> Vec<Vec<T>>
where
    T: Clone + Hash + Eq + Debug,
{
    let mut generations: Vec<Vec<T>> = Vec::new();
    let mut indegree_map: HashMap<T, usize> = HashMap::new();
    let mut zero_indegree: Vec<T> = Vec::new();

    for (k, v) in g.in_degree_map().iter() {
        match v {
            0 => zero_indegree.push(k.clone()),
            _ => {
                indegree_map.insert(k.clone(), v.clone());
            }
        };
    }

    while !zero_indegree.is_empty() {
        let this_generation = zero_indegree.clone();
        zero_indegree = vec![];

        for node in this_generation.iter() {
            for child in g.adj(node).expect("Node does not exist") {
                indegree_map.entry(child.clone()).and_modify(|n| *n -= 1);
                if *indegree_map.get(&child).unwrap() == 0 {
                    zero_indegree.push(child.clone());
                    indegree_map.remove(child);
                }
            }
        }
        generations.push(this_generation);
    }
    generations
}

pub fn topological_sort<T>(g: &Graph<T, Directed>) -> Vec<T>
where
    T: Clone + Hash + Eq + Debug,
{
    topological_generations(g).into_iter().flatten().collect()
}

fn bfs_util<T, G>(g: &Graph<T, G>, source: T, target: T) -> Option<(usize, HashMap<T, T>)>
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
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

pub fn shortest_path<T, G>(g: &Graph<T, G>, source: T, target: T) -> Option<Vec<T>>
where
    T: Clone + Hash + Eq + Debug,
    G: GraphType,
{
    match bfs_util(g, source.clone(), target.clone()) {
        Some((_, mut previous)) => {
            let path = build_path::<T>(&mut previous, source, target);
            match path.len() {
                0 => None,
                _ => Some(path.clone()),
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        let g: Graph<i8> = Graph::new();
        assert_eq!(g.nodes::<HashSet<_>>(), HashSet::new());
    }

    #[test]
    fn add_nodes() {
        let mut g: Graph<i8> = Graph::new();
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);
        assert_eq!(g.nodes::<HashSet<_>>(), HashSet::from([1, 2, 3]));
    }

    #[test]
    fn add_edges() {
        let mut g: Graph<i8> = Graph::new();
        g.add_edge(1, 2);
        assert_eq!(g.nodes::<HashSet<_>>(), HashSet::from([1, 2]));
        assert_eq!(*g.adj(&1).unwrap(), HashSet::from([2]));
        assert_eq!(*g.adj(&2).unwrap(), HashSet::from([1]));
    }

    #[test]
    fn no_adj() {
        let g: Graph<i8> = Graph::new();
        assert!(g.adj(&2).is_none());
    }
}
