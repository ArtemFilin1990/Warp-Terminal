/// A single completion suggestion.
#[derive(Debug, Clone, PartialEq)]
pub struct Completion {
    pub value: String,
    pub description: String,
}

/// Provides completion suggestions based on a prefix.
pub struct AutocompleteEngine {
    /// Built-in shell commands available for completion.
    commands: Vec<Completion>,
}

impl AutocompleteEngine {
    pub fn new() -> Self {
        let commands = vec![
            Completion { value: "cd".into(), description: "Change directory".into() },
            Completion { value: "ls".into(), description: "List directory contents".into() },
            Completion { value: "echo".into(), description: "Print text to stdout".into() },
            Completion { value: "export".into(), description: "Set environment variable".into() },
            Completion { value: "source".into(), description: "Execute commands from file".into() },
            Completion { value: "grep".into(), description: "Search text patterns".into() },
            Completion { value: "find".into(), description: "Find files".into() },
            Completion { value: "cargo".into(), description: "Rust package manager".into() },
            Completion { value: "git".into(), description: "Version control".into() },
        ];
        AutocompleteEngine { commands }
    }

    /// Return completions whose `value` starts with `prefix`.
    pub fn complete(&self, prefix: &str) -> Vec<&Completion> {
        self.commands
            .iter()
            .filter(|c| c.value.starts_with(prefix))
            .collect()
    }
}

impl Default for AutocompleteEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_matches_prefix() {
        let engine = AutocompleteEngine::new();
        let results = engine.complete("ca");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, "cargo");
    }

    #[test]
    fn test_complete_no_match() {
        let engine = AutocompleteEngine::new();
        let results = engine.complete("zzz");
        assert!(results.is_empty());
    }

    #[test]
    fn test_complete_empty_prefix_returns_all() {
        let engine = AutocompleteEngine::new();
        let results = engine.complete("");
        assert_eq!(results.len(), engine.commands.len());
    }
}
