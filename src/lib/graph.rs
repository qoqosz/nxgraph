//! Definition of graphs.
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// An undirected graph type.
#[derive(Debug)]
pub struct Undirected {}

/// A directed graph type.
#[derive(Debug)]
pub struct Directed {}

/// A graph type.
pub trait GraphType {}

impl GraphType for Undirected {}
impl GraphType for Directed {}

/// A graph object.
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

/// An alias for a directed graph.
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
