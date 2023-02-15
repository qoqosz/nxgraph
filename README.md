# nxgraph
Simple Rust graph library inspired by NetworkX's API.

## Simple directed graph
```
1 - 2 - 3 - 5      6
 \- 4 -/
``` 
Snippet from `examples.rs` demonstrating how to construct a graph and access its basic properties:
```rust
use nxgraph::{topological_sort, DiGraph};
use std::collections::HashSet;

fn main() {
    let mut g: DiGraph<u8> = DiGraph::new();
    g.add_edges_from(vec![(1, 2), (1, 4), (2, 3), (3, 5), (4, 3)]);
    g.add_node(6);

    println!("nodes={:?}", g.nodes::<HashSet<_>>());
    println!("edges={:?}", g.edges::<HashSet<(_, _)>>());
    println!("topological sort={:?}", topological_sort(&g));
}

```
output:
```
nodes={6, 2, 1, 4, 5, 3}
edges={(4, 3), (3, 5), (2, 3), (1, 2), (1, 4)}
topological sort=[6, 1, 4, 2, 3, 5]
```
