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
