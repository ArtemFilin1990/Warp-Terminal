pub mod file_tree;
pub mod lsp;

pub use file_tree::FileTree;
pub use lsp::LspClient;

use std::path::PathBuf;

/// An open file in the editor with its content and unsaved-changes flag.
#[derive(Debug, Clone)]
pub struct OpenFile {
    pub path: PathBuf,
    pub content: String,
    pub modified: bool,
}

impl OpenFile {
    pub fn new(path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        OpenFile {
            path: path.into(),
            content: content.into(),
            modified: false,
        }
    }

    /// Apply a text replacement and mark the file as modified.
    pub fn apply_edit(&mut self, range_start: usize, range_end: usize, replacement: &str) {
        self.content.replace_range(range_start..range_end, replacement);
        self.modified = true;
    }
}

/// The code editor state: open files, active file, and LSP client.
pub struct CodeEditor {
    pub files: Vec<OpenFile>,
    pub active_index: Option<usize>,
    pub lsp: LspClient,
    pub file_tree: FileTree,
}

impl CodeEditor {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        CodeEditor {
            files: Vec::new(),
            active_index: None,
            lsp: LspClient::new(),
            file_tree: FileTree::new(root),
        }
    }

    /// Open a file by path, reading its content from disk.
    pub fn open_file(&mut self, path: impl Into<PathBuf>) -> std::io::Result<()> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        self.lsp.open_document(&path);
        let idx = self.files.len();
        self.files.push(OpenFile::new(path, content));
        self.active_index = Some(idx);
        Ok(())
    }

    /// Return a reference to the currently active file.
    pub fn active_file(&self) -> Option<&OpenFile> {
        self.active_index.and_then(|i| self.files.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file_applies_edit() {
        let mut f = OpenFile::new("/tmp/test.rs", "Hello World");
        f.apply_edit(6, 11, "Rust");
        assert_eq!(f.content, "Hello Rust");
        assert!(f.modified);
    }
}
