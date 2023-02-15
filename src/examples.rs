use nxgraph::*;
use std::collections::HashSet;

fn undirected_graph() {
    let mut g: Graph<u8> = Graph::new();
    g.add_edges_from(vec![(1, 2), (2, 3), (3, 5), (1, 4), (4, 5)]);
    g.add_node(6);

    println!("graph={:?}", g);
    println!("nodes={:?}", g.nodes::<HashSet<_>>());
    println!("edges={:?}", g.edges::<HashSet<(_, _)>>());
    println!(
        "shortest path from 1 to 5={:?}",
        shortest_path(&g, 1, 5).unwrap()
    );
    println!("shortest path from 1 to 6={:?}", shortest_path(&g, 1, 6));
    println!(
        "shortest path length from 1 to 5={:?}",
        shortest_path_length(&g, 1, 5).unwrap()
    );
}

fn directed_graph() {
    let mut g: DiGraph<u8> = DiGraph::new();
    g.add_edges_from(vec![(1, 2), (2, 3), (3, 5), (1, 4), (4, 5)]);
    g.add_node(6);

    println!("graph={:?}", g);
    println!("nodes={:?}", g.nodes::<HashSet<_>>());
    println!("edges={:?}", g.edges::<HashSet<(_, _)>>());
    println!("in degree map={:?}", g.in_degree_map());
    println!("topological sort={:?}", topological_sort(&g));
    println!(
        "shortest path from 1 to 5={:?}",
        shortest_path(&g, 1, 5).unwrap()
    );
    println!(
        "shortest path length from 1 to 5={:?}",
        shortest_path_length(&g, 1, 5).unwrap()
    );
}

fn main() {
    println!("Undirected graph");
    println!("================");
    undirected_graph();

    println!("\n\nDirected graph");
    println!("==============");
    directed_graph();
}
