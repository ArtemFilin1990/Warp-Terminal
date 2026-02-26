use std::path::{Path, PathBuf};

/// A node in the file tree.
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
    pub expanded: bool,
}

impl FileNode {
    /// Create a leaf file node.
    pub fn file(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        FileNode {
            name: name.into(),
            path: path.into(),
            is_dir: false,
            children: Vec::new(),
            expanded: false,
        }
    }

    /// Create a directory node.
    pub fn dir(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        FileNode {
            name: name.into(),
            path: path.into(),
            is_dir: true,
            children: Vec::new(),
            expanded: false,
        }
    }
}

/// Manages the file tree for the editor pane.
pub struct FileTree {
    pub root: FileNode,
    /// Index (depth-first order) of the currently selected node.
    pub selected_index: usize,
}

impl FileTree {
    pub fn new(root_path: impl Into<PathBuf>) -> Self {
        let root_path = root_path.into();
        let name = root_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "/".into());
        FileTree {
            root: FileNode::dir(name, root_path),
            selected_index: 0,
        }
    }

    /// Populate `node` by reading the directory from the file system.
    pub fn load_dir(node: &mut FileNode) -> std::io::Result<()> {
        if !node.is_dir {
            return Ok(());
        }
        node.children.clear();
        let mut entries: Vec<_> = std::fs::read_dir(&node.path)?
            .filter_map(|e| e.ok())
            .collect();
        entries.sort_by_key(|e| {
            let is_file = e.file_type().map(|t| t.is_file()).unwrap_or(false);
            (is_file, e.file_name())
        });
        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let child = if is_dir {
                FileNode::dir(name, path)
            } else {
                FileNode::file(name, path)
            };
            node.children.push(child);
        }
        node.expanded = true;
        Ok(())
    }

    /// Toggle expand/collapse for the node at the given path.
    pub fn toggle_dir(node: &mut FileNode, target: &Path) -> bool {
        if node.path == target && node.is_dir {
            if node.expanded {
                node.expanded = false;
                node.children.clear();
            } else {
                let _ = Self::load_dir(node);
            }
            return true;
        }
        for child in &mut node.children {
            if Self::toggle_dir(child, target) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_node_creation() {
        let node = FileNode::file("main.rs", "/src/main.rs");
        assert_eq!(node.name, "main.rs");
        assert!(!node.is_dir);
    }

    #[test]
    fn test_dir_node_creation() {
        let node = FileNode::dir("src", "/src");
        assert!(node.is_dir);
        assert!(!node.expanded);
    }

    #[test]
    fn test_file_tree_root_name() {
        let tree = FileTree::new("/home/user/project");
        assert_eq!(tree.root.name, "project");
        assert!(tree.root.is_dir);
    }
}
