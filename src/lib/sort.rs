//! Graph sorting utilities.
use crate::graph::{Directed, Graph};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct CycleError;

impl std::error::Error for CycleError {}

impl Display for CycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A cycle has been detected in a graph")
    }
}

type Result<T> = std::result::Result<T, CycleError>;

pub fn topological_generations<T>(g: &Graph<T, Directed>) -> Result<Vec<Vec<T>>>
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
            for child in g.adj(node).expect("No such node in a graph") {
                indegree_map.entry(child.clone()).and_modify(|n| *n -= 1);
                if *indegree_map.get(&child).unwrap() == 0 {
                    zero_indegree.push(child.clone());
                    indegree_map.remove(child);
                }
            }
        }
        generations.push(this_generation);
    }

    if !indegree_map.is_empty() {
        return Err(CycleError);
    }

    Ok(generations)
}

pub fn topological_sort<T>(g: &Graph<T, Directed>) -> Result<Vec<T>>
where
    T: Clone + Hash + Eq + Debug,
{
    match topological_generations(g) {
        Ok(gens) => {
            let sorted = gens.into_iter().flatten().collect();
            Ok(sorted)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::topological_generations;

    use super::*;

    fn simple_graph() -> Graph<i8, Directed> {
        let mut g: Graph<i8, Directed> = Graph::new();
        g.add_edges_from(vec![(1, 2), (2, 3), (3, 4), (1, 5), (5, 4), (4, 6)]);
        g.add_node(7);
        g
    }

    #[test]
    fn test_topological_generations() {
        let g = simple_graph();
        let actual = topological_generations(&g)
            .ok()
            .unwrap()
            .into_iter()
            .map(|mut gen| {
                gen.sort();
                gen
            })
            .collect::<Vec<Vec<i8>>>();
        let expected = vec![vec![1, 7], vec![2, 5], vec![3], vec![4], vec![6]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_topological_sort() {
        let g = simple_graph();
        let actual = topological_sort(&g).ok().unwrap();
        let expected = vec![
            vec![1, 7, 2, 5, 3, 4, 6],
            vec![7, 1, 2, 5, 3, 4, 6],
            vec![1, 7, 5, 2, 3, 4, 6],
            vec![7, 1, 5, 2, 3, 4, 6],
        ];
        assert!(expected.contains(&actual));
    }

    #[test]
    fn cycle_error() {
        let mut g: Graph<i8, Directed> = Graph::new();
        g.add_edges_from(vec![(1, 2), (2, 3), (3, 1)]);
        assert!(topological_sort(&g).is_err());
    }
}
