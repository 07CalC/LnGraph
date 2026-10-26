use std::path::PathBuf;

use crate::node::Node;

pub struct Edge {
    type_: String,
    properties: serde_json::Value,
    from: Box<Node>,
    to: Box<Node>,
}

impl Edge {
    pub fn new(
        type_: &str,
        properties: serde_json::Value,
        from: Box<Node>,
        to: Box<Node>,
        root: &PathBuf,
    ) -> Result<Self, String> {
        println!("root: {}", root.to_str().unwrap());
        if !root.is_dir() {
            return Err(format!("{} is not a directory", root.display()));
        }

        let edge_dir = from.path(root).join(type_).join(to.id());

        if edge_dir.exists() {
            return Err(format!("Edge {} already exists", edge_dir.display()));
        }

        std::fs::create_dir_all(&edge_dir)
            .map_err(|e| format!("Failed to create edge directory: {}", e))?;

        std::fs::write(
            edge_dir.join("properties.json"),
            serde_json::to_string_pretty(&properties)
                .map_err(|e| format!("Failed to serialize properties: {}", e))?,
        )
        .map_err(|e| format!("Failed to write properties file: {}", e))?;

        std::os::unix::fs::symlink(to.path(root), edge_dir.join("target"))
            .map_err(|e| format!("Failed to create symlink: {}", e))?;

        Ok(Edge {
            type_: type_.to_string(),
            properties,
            from,
            to,
        })
    }
}
