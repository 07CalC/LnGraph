use std::path::PathBuf;

mod graph;
mod node;
fn main() {
    let mut graph = graph::Graph::new(Some(PathBuf::from("graph_data")));
    graph
        .create_node(
            "Person".to_string(),
            serde_json::json!({"name": "Alice", "age": 30}),
        )
        .unwrap();

    graph
        .create_node(
            "Person".to_string(),
            serde_json::json!({"name": "Bob", "age": 25}),
        )
        .unwrap();
}
