//! Graph sorting utilities.
use crate::graph::{Directed, Graph};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

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
    generations
}

pub fn topological_sort<T>(g: &Graph<T, Directed>) -> Vec<T>
where
    T: Clone + Hash + Eq + Debug,
{
    topological_generations(g).into_iter().flatten().collect()
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
        let actual = topological_sort(&g);
        let expected = vec![vec![1, 7, 2, 5, 3, 4, 6], vec![7, 1, 2, 5, 3, 4, 6]];
        assert!((actual == expected[0]) || (actual == expected[1]));
    }
}
