use std::path::PathBuf;

/// A resource that can be referenced via @-selection.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionResource {
    /// A file path on disk.
    File(PathBuf),
    /// Raw text content, e.g. selected terminal output.
    Text(String),
    /// A base64-encoded image captured from the terminal.
    Image { data: String, mime_type: String },
}

/// An @-selection entry binding a user-specified label to a resource.
#[derive(Debug, Clone)]
pub struct Selection {
    /// The @label used to reference this resource in prompts (without the `@`).
    pub label: String,
    pub resource: SelectionResource,
}

/// Registry of active @-selections for the current agent interaction.
pub struct SelectionRegistry {
    selections: Vec<Selection>,
}

impl SelectionRegistry {
    pub fn new() -> Self {
        SelectionRegistry { selections: Vec::new() }
    }

    /// Register a new @-selection.
    pub fn add(&mut self, label: impl Into<String>, resource: SelectionResource) {
        self.selections.push(Selection {
            label: label.into(),
            resource,
        });
    }

    /// Look up a selection by label.
    pub fn get(&self, label: &str) -> Option<&Selection> {
        self.selections.iter().find(|s| s.label == label)
    }

    /// Remove a selection by label. Returns `true` if it existed.
    pub fn remove(&mut self, label: &str) -> bool {
        if let Some(pos) = self.selections.iter().position(|s| s.label == label) {
            self.selections.remove(pos);
            true
        } else {
            false
        }
    }

    /// Return all registered selections.
    pub fn all(&self) -> &[Selection] {
        &self.selections
    }

    /// Expand @-references in `prompt` text by replacing `@label` tokens
    /// with a brief description of the referenced resource.
    pub fn expand_prompt(&self, prompt: &str) -> String {
        let mut result = prompt.to_owned();
        for sel in &self.selections {
            let token = format!("@{}", sel.label);
            let replacement = match &sel.resource {
                SelectionResource::File(p) => format!("[file: {}]", p.display()),
                SelectionResource::Text(t) => format!("[text: {}]", t),
                SelectionResource::Image { mime_type, .. } => {
                    format!("[image: {}]", mime_type)
                }
            };
            result = result.replace(&token, &replacement);
        }
        result
    }
}

impl Default for SelectionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get() {
        let mut reg = SelectionRegistry::new();
        reg.add("myfile", SelectionResource::File("/tmp/foo.rs".into()));
        let sel = reg.get("myfile").unwrap();
        assert_eq!(sel.label, "myfile");
    }

    #[test]
    fn test_remove() {
        let mut reg = SelectionRegistry::new();
        reg.add("x", SelectionResource::Text("hello".into()));
        assert!(reg.remove("x"));
        assert!(!reg.remove("x")); // already removed
    }

    #[test]
    fn test_expand_prompt() {
        let mut reg = SelectionRegistry::new();
        reg.add("f", SelectionResource::File("/src/main.rs".into()));
        let expanded = reg.expand_prompt("Review @f please");
        assert_eq!(expanded, "Review [file: /src/main.rs] please");
    }

    #[test]
    fn test_expand_prompt_image() {
        let mut reg = SelectionRegistry::new();
        reg.add("img", SelectionResource::Image {
            data: "base64data".into(),
            mime_type: "image/png".into(),
        });
        let expanded = reg.expand_prompt("Describe @img");
        assert_eq!(expanded, "Describe [image: image/png]");
    }
}
