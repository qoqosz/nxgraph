# nxgraph
Simple Rust graph library inspired by NetworkX's API.

## Simple directed graph
```
1 - 2 - 3 - 5      6
 '--- 4 ---'
```
Snippet from `examples.rs` demonstrating how to construct a graph and access its basic properties:
```rust
use nxgraph::{topological_sort, DiGraph};
use std::collections::HashSet;

fn main() {
    let mut g: DiGraph<u8> = DiGraph::new();
    g.add_edges_from(vec![(1, 2), (2, 3), (3, 5), (1, 4), (4, 5)]);
    g.add_node(6);

    println!("nodes={:?}", g.nodes::<HashSet<_>>());
    println!("edges={:?}", g.edges::<HashSet<(_, _)>>());
    println!("topological sort={:?}", topological_sort(&g));
    println!(
        "shortest path from 1 to 5={:?}",
        shortest_path(&g, 1, 5).unwrap()
    );
}

```
output:
```
nodes={3, 4, 5, 1, 6, 2}
edges={(1, 2), (4, 5), (2, 3), (1, 4), (3, 5)}
topological sort=[1, 6, 2, 4, 3, 5]
shortest path from 1 to 5=[1, 4, 5]
```
