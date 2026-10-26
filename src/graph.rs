#![allow(dead_code)]
use std::path::PathBuf;

use crate::node::Node;

pub struct Graph {
    root: Option<PathBuf>,
}

impl Graph {
    pub fn new(root: Option<PathBuf>) -> Self {
        if let Some(ref path) = root {
            if !path.is_dir() {
                std::fs::create_dir_all(path).expect("Failed to create directory");
            } else {
            }
            return Graph {
                root: Some(Self::absoulte_path(path).expect("Failed to get absolute path")),
            };
        }
        Graph { root: None }
    }

    pub fn open(&mut self, root: PathBuf) -> Result<(), String> {
        if root.is_dir() {
            self.root = Some(root);
            Ok(())
        } else {
            Err(format!("{} is not a directory", root.display()))
        }
    }
    pub fn get_root(&self) -> Option<&PathBuf> {
        self.root.as_ref()
    }
    pub fn close(&mut self) {
        self.root = None;
    }
    pub fn create_node(
        &self,
        type_: String,
        properties: serde_json::Value,
    ) -> Result<Node, String> {
        if let Some(ref root) = self.root {
            let id = uuid::Uuid::new_v4().to_string();
            let node = Node::new(id.clone(), type_, properties, root)?;
            Ok(node)
        } else {
            Err("Graph is not open".to_string())
        }
    }

    pub fn create_edge(
        &self,
        type_: String,
        properties: serde_json::Value,
        from: Box<Node>,
        to: Box<Node>,
    ) -> Result<crate::edge::Edge, String> {
        if let Some(ref root) = self.root {
            let edge = crate::edge::Edge::new(type_.as_str(), properties, from, to, root)?;
            Ok(edge)
        } else {
            Err("Graph is not open".to_string())
        }
    }

    fn absoulte_path(path: &PathBuf) -> Result<PathBuf, String> {
        path.canonicalize()
            .map_err(|e| format!("Failed to get absolute path: {}", e))
    }
}
