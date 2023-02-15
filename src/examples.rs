use nxgraph::graph::*;
use std::collections::HashSet;

fn main() {
    let mut g: DiGraph<u8> = DiGraph::new();
    g.add_edges_from(vec![(1, 2), (1, 4), (2, 3), (3, 5), (4, 3)]);
    g.add_node(6);

    println!("graph={:?}", g);
    println!("nodes={:?}", g.nodes::<HashSet<_>>());
    println!("edges={:?}", g.edges::<HashSet<(_, _)>>());
}
