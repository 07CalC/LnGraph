#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use std::path::PathBuf;

mod edge;
mod graph;
mod node;
fn main() {
    let mut graph = graph::Graph::new(Some(PathBuf::from("graph_data")));
    let alice = graph
        .create_node(
            "Person".to_string(),
            serde_json::json!({"name": "Alice", "age": 30}),
        )
        .unwrap();

    let bob = graph
        .create_node(
            "Person".to_string(),
            serde_json::json!({"name": "Bob", "age": 25}),
        )
        .unwrap();

    match graph.create_edge(
        "Follows".to_string(),
        serde_json::json!({
            "since": "2024-01-01"
        }),
        Box::new(alice),
        Box::new(bob),
    ) {
        Ok(edge) => {
            println!("Edge created successfully!");
        }
        Err(e) => {
            println!("Failed to create edge: {}", e);
        }
    }
}
