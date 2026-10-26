use std::path::PathBuf;

pub struct Node {
    id: String,
    type_: String,
    properties: serde_json::Value,
}

impl Node {
    pub fn new(
        id: String,
        type_: String,
        properties: serde_json::Value,
        root: &PathBuf,
    ) -> Result<Self, String> {
        if !root.is_dir() {
            return Err(format!("{} is not a directory", root.display()));
        }

        let node_dir = root.join(&type_).join(&id);

        if node_dir.exists() {
            return Err(format!("Node {} already exists", node_dir.display()));
        }

        std::fs::create_dir_all(&node_dir)
            .map_err(|e| format!("Failed to create node directory: {}", e))?;

        std::fs::write(
            node_dir.join("properties.json"),
            serde_json::to_string_pretty(&properties)
                .map_err(|e| format!("Failed to serialize properties: {}", e))?,
        )
        .map_err(|e| format!("Failed to write properties file: {}", e))?;

        Ok(Node {
            id,
            type_,
            properties,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn node_type(&self) -> &str {
        &self.type_
    }

    pub fn properties(&self) -> &serde_json::Value {
        &self.properties
    }
}
